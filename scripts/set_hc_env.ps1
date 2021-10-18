$Env:OLTP_TONIC_ENDPOINT = "grpc://api.honeycomb.io:443/"
$Env:OLTP_TONIC_X_HONEYCOMB_TEAM = "453406d2a77ff2100066771a9cb2aa61"
$Env:OLTP_TONIC_X_HONEYCOMB_DATASET = "dataset"
cd G:\Projects\rust\zero2prod
cargo test
