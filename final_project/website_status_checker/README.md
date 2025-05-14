Website Status Checker

Build Instructions:
cargo build --release

To run program:
For single URL use:
./target/release/website_status_checker (URL of choosing)
For multiple URL's:
1.)Enter URL into txt file, in this case I used sites.txt where it includes the sites to be checked
2.)In terminal in proper location : ./target/release/website-status-checker-rust --file sites.txt

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
