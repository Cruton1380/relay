// apps/server/src/relay_physics/route_generator.rs
// PR #8: Route Generation Utilities
//
// Generates deterministic routes for shipments (drones) to follow.
// MVP: Simple great circle routes (origin -> destination)
// Future: Multi-waypoint, terrain avoidance, weather-based routing

use crate::relay_physics::{GeoPosition, GeoAnchor, CarrierType};

/// Generate a simple 2-waypoint route from origin to destination
/// 
/// This is the MVP implementation: straight line at fixed altitude.
/// Returns a vec with 2 positions (start and end).
///
/// # Determinism
/// Same origin + destination + carrier -> same route (no randomness)
pub fn generate_simple_route(
    origin: &GeoAnchor,
    destination: &GeoAnchor,
    carrier_type: CarrierType,
) -> Vec<GeoPosition> {
    let altitude = get_carrier_altitude(carrier_type);
    
    vec![
        GeoPosition {
            lat: origin.lat,
            lon: origin.lon,
            alt: altitude,
        },
        GeoPosition {
            lat: destination.lat,
            lon: destination.lon,
            alt: altitude,
        },
    ]
}

/// Get standard altitude for carrier type
fn get_carrier_altitude(carrier_type: CarrierType) -> f64 {
    match carrier_type {
        CarrierType::Drone => 100.0,   // 100 meters
        CarrierType::Ground => 0.0,    // Ground level
        CarrierType::Air => 10000.0,   // 10 km
    }
}

/// Calculate estimated travel time in seconds
/// 
/// Uses simple distance / speed calculation.
/// Future: account for terrain, weather, traffic
pub fn estimate_travel_time(
    origin: &GeoAnchor,
    destination: &GeoAnchor,
    carrier_type: CarrierType,
) -> u64 {
    let distance_km = calculate_great_circle_distance(origin, destination);
    let speed_km_per_hour = get_carrier_speed(carrier_type);
    let hours = distance_km / speed_km_per_hour;
    let seconds = (hours * 3600.0).ceil() as u64;
    
    // Minimum 60 seconds (for demo/testing)
    seconds.max(60)
}

/// Get standard speed for carrier type (km/h)
fn get_carrier_speed(carrier_type: CarrierType) -> f64 {
    match carrier_type {
        CarrierType::Drone => 60.0,    // 60 km/h (~37 mph)
        CarrierType::Ground => 40.0,   // 40 km/h (~25 mph)
        CarrierType::Air => 800.0,     // 800 km/h (~500 mph)
    }
}

/// Calculate great circle distance between two points (in kilometers)
/// 
/// Uses Haversine formula:
/// https://en.wikipedia.org/wiki/Haversine_formula
pub fn calculate_great_circle_distance(
    origin: &GeoAnchor,
    destination: &GeoAnchor,
) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    
    let lat1_rad = origin.lat.to_radians();
    let lat2_rad = destination.lat.to_radians();
    let delta_lat = (destination.lat - origin.lat).to_radians();
    let delta_lon = (destination.lon - origin.lon).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    EARTH_RADIUS_KM * c
}

/// Generate multi-waypoint route (future enhancement)
/// 
/// Could add intermediate waypoints to:
/// - Avoid obstacles (buildings, terrain)
/// - Follow airways/roads
/// - Route around weather
/// - Optimize for fuel/time
#[allow(dead_code)]
pub fn generate_waypoint_route(
    origin: &GeoAnchor,
    destination: &GeoAnchor,
    carrier_type: CarrierType,
    _waypoint_count: usize,
) -> Vec<GeoPosition> {
    // For now, just call simple route
    // TODO: Add intermediate waypoints
    generate_simple_route(origin, destination, carrier_type)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    fn nyc_anchor() -> GeoAnchor {
        GeoAnchor {
            lat: 40.7128,
            lon: -74.0060,
            alt: None,
        }
    }
    
    fn la_anchor() -> GeoAnchor {
        GeoAnchor {
            lat: 34.0522,
            lon: -118.2437,
            alt: None,
        }
    }
    
    #[test]
    fn test_simple_route_generation() {
        let origin = nyc_anchor();
        let destination = la_anchor();
        
        let route = generate_simple_route(&origin, &destination, CarrierType::Drone);
        
        assert_eq!(route.len(), 2);
        assert_eq!(route[0].lat, origin.lat);
        assert_eq!(route[0].lon, origin.lon);
        assert_eq!(route[0].alt, 100.0); // Drone altitude
        
        assert_eq!(route[1].lat, destination.lat);
        assert_eq!(route[1].lon, destination.lon);
        assert_eq!(route[1].alt, 100.0);
    }
    
    #[test]
    fn test_carrier_altitudes() {
        let origin = nyc_anchor();
        let dest = la_anchor();
        
        let drone_route = generate_simple_route(&origin, &dest, CarrierType::Drone);
        assert_eq!(drone_route[0].alt, 100.0);
        
        let ground_route = generate_simple_route(&origin, &dest, CarrierType::Ground);
        assert_eq!(ground_route[0].alt, 0.0);
        
        let air_route = generate_simple_route(&origin, &dest, CarrierType::Air);
        assert_eq!(air_route[0].alt, 10000.0);
    }
    
    #[test]
    fn test_great_circle_distance() {
        let nyc = nyc_anchor();
        let la = la_anchor();
        
        // NYC to LA is approximately 3,944 km
        let distance = calculate_great_circle_distance(&nyc, &la);
        assert!(distance > 3900.0 && distance < 4000.0);
    }
    
    #[test]
    fn test_travel_time_estimation() {
        let nyc = nyc_anchor();
        let la = la_anchor();
        
        // Drone at 60 km/h: ~3944 km / 60 = ~66 hours = ~237,000 seconds
        let drone_time = estimate_travel_time(&nyc, &la, CarrierType::Drone);
        assert!(drone_time > 200_000 && drone_time < 250_000);
        
        // Air at 800 km/h: ~3944 km / 800 = ~5 hours = ~18,000 seconds
        let air_time = estimate_travel_time(&nyc, &la, CarrierType::Air);
        assert!(air_time > 15_000 && air_time < 20_000);
    }
    
    #[test]
    fn test_minimum_travel_time() {
        // Very short distance (same point)
        let origin = nyc_anchor();
        let destination = nyc_anchor();
        
        let time = estimate_travel_time(&origin, &destination, CarrierType::Drone);
        assert_eq!(time, 60); // Minimum 60 seconds
    }
    
    #[test]
    fn test_determinism() {
        let origin = nyc_anchor();
        let dest = la_anchor();
        
        // Same inputs should produce identical routes
        let route1 = generate_simple_route(&origin, &dest, CarrierType::Drone);
        let route2 = generate_simple_route(&origin, &dest, CarrierType::Drone);
        
        assert_eq!(route1.len(), route2.len());
        for (p1, p2) in route1.iter().zip(route2.iter()) {
            assert_eq!(p1.lat, p2.lat);
            assert_eq!(p1.lon, p2.lon);
            assert_eq!(p1.alt, p2.alt);
        }
    }
}
