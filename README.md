# Solari

Solari is a high-performance transit routing engine built using the [RAPTOR algorithm](https://www.microsoft.com/en-us/research/wp-content/uploads/2012/01/raptor_alenex.pdf), optimized for lightweight, global-scale public transit routing. Designed to serve developers building applications requiring fast and resource-efficient transit planning (e.g., maps apps, trip-planning APIs), it avoids heavy preprocessing steps while supporting planet-scale coverage through memory-mapped timetables.

## Key Features
- **Planet-Scale Coverage**:
  - Memory-mapped timetable data allows a single instance to handle global networks with minimal RAM usage (via `memmap2`).

- **Multi-Agency Support**:
  - Load multiple GTFS feeds from a directory for seamless cross-agency routing.

- **Timezone Awareness**:
  - Automatically handles timezone conversions based on GTFS feed data. Developers are responsible for converting epoch timestamps to local time in their app layer.

- **HTTP API Endpoint**:
  ```http
  POST /v1/plan
  ```
  Example request:
  ```bash
  curl -d '{"from":{"lat":47.679591,"lon":-122.356388},"to":{"lat":47.616440,"lon":-122.320440},"start_at":1742845000000}' \
       https://transit.maps.earth/v1/plan
  ```

- **GTFS Compatibility**:
  - Supports modern GTFS feeds via the `gtfs-structures` crate.
  - No real-time (GTFS-RT) support yet; prioritized roadmap features include alerts and delays.

## Getting Started

### Prerequisites
1. **Rust** (`rustc >= 1.86` tested).
2. **OpenSSL development package**: Install via your OS's package manager (e.g., `libssl-dev` on Ubuntu).

### TODO: Quickstart

It used to be pretty simple to get a Solari instance up and running but we added support for [pedestrian routing](https://github.com/ellenhp/solari/pull/16) during transfers which complicated setup. Awaiting new documentation.

## Architecture
- **RAPTOR Algorithm**: Implements all pruning rules from the original paper for optimal performance.
- **Memory Mapping**: Uses `memmap2` to load timetable data directly from disk, enabling fast access without RAM overhead.

## Roadmap
- **GTFS-RT Support** (priority order):
  1. Service alerts and closures
  2. Real-time delays
- **Performance Quantification**: Come up with better benchmarks against MOTIS and OpenTripPlanner.
- **rRAPTOR Implementation**: Long-term goal for multi-departure-time routing.
- **Documentation**: Ongoing work to finalize API response formats and provide detailed guides.

## Contributing
- Solari is in active development; contributions (documentation, testing, or features) are welcome.
- Check the repository's issue tracker for tasks, but note there are no formal contribution guidelines yet.

## Known Limitations
- **No Real-Time Updates**: Only static GTFS feeds supported currently.
- **API Stability**: The `/v1/plan` response format may evolve as documentation finalizes, but no compatibility breaking changes to the v1 endpoint after the initial release.

## When to Use Solari?
You may want to use this project if you need:
- Fast, lightweight routing for global-scale transit networks on modest hardware.
- A minimal API layer that integrates easily with modern web stacks (geocoding, map rendering handled externally).

Avoid if you require:
- Full-featured trip-planning like OpenTripPlanner's extensive customization or real-time capabilities.

## License
[Apache-2.0](LICENSE)
