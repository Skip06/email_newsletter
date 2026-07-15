#building stage

FROM lukemathwalker/cargo-chef:latest-rust-1.97.0 as chef
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y


FROM chef as planner
# Copy all files from our working environment to our Docker image
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
#forcing sqlx to look at the saved metadata instead of trying to query a live database
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release


#runtime stage

FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies /// debian doesnot work without openssl
# Install ca-certificates - it is needed to verify TLS certificates when establishing HTTPS connections
#uItpdate downloads the latest list of available packages from Debian servers.
RUN apt-get update -y \   
&& apt-get install -y --no-install-recommends openssl ca-certificates \
# Clean up
&& apt-get autoremove -y \
#removes those downloaded archives. git.deb like zip extract delete zip
&& apt-get clean -y \
#This removes the package index downloaded by apt-get update.
&& rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/actixweb_email_newsletter actixweb_email_newsletter
# We need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./actixweb_email_newsletter"]


#out final image has only runtime image.
#if debian does not have openssl application crashes