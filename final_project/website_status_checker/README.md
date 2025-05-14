Website Status Checker

Build Instructions:
cargo build --release

To run program:
./target/release/website_status_checker (URL of choosing)

Output:
The description regarding URL will be output to 'status.json' in which will describe the name
of URL, status, response time(miliseconds), and timestamp.

Example of output in status.json:
[
  {
    "url": "https://example.com",
    "status": 200,
    "response_time_ms": 74,
    "timestamp": 1747247724
  }
]
