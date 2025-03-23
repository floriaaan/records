#!/usr/bin/env sh
set -e

# filepath: /Users/floriaaan/dev/records-rust/docker-entrypoint.sh

# Load environment variables from .env if it exists, but do not override already set values
if [ -f .env ]; then
    while IFS='=' read -r key value; do
        # Ignore comments or empty lines
        if [[ "$key" =~ ^#.*$ || -z "$key" ]]; then
            continue
        fi
        # Only export the variable if not already set
        if [ -z "${!key}" ]; then
            export "$key=$value"
        fi
    done < .env
fi

if [ -z "$DATABASE_URL" ]; then
    echo "DATABASE_URL is not set. Exiting."
    exit 1
fi

echo "DATABASE_URL is set to $DATABASE_URL"

# Parse DATABASE_URL assuming the format: protocol://user:pass@host:port/dbname
db_url="$DATABASE_URL"
# Remove protocol part
host_port_part="${db_url#*://}"
# Remove user credentials if present
host_port_part="${host_port_part#*@}"
# Extract host and port
host="${host_port_part%%:*}"
port_with_db="${host_port_part#*:}"
port="${port_with_db%%/*}"

echo "Waiting for database at $host:$port to be available..."

retry_count=0
until nc -z "$host" "$port" 2>/dev/null; do
    ((retry_count++))
    if [ "$retry_count" -ge 5 ]; then
        echo "Database not available after 5 attempts. Exiting."
        exit 1
    fi
    echo "Database not available - waiting..."
    sleep 1
done
echo "Database is up!"

# Run migration script
echo "Running migrations..."
/app/sqlx migrate run

# Run the application in release mode
echo "Starting application..."

# Ensure the binary is executable
chmod +x /app/records

# Start the application
exec /app/records
