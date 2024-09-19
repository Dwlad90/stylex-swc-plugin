#!/usr/bin/env sh

handle_error() {
  echo "Error: $1"
  exit 1
}

# Function to increment the version tag
increment_version() {
  # shellcheck disable=SC3043
  local version="$1"
  # shellcheck disable=SC3043
  local release_type="$2"

  if [ -z "$release_type" ]; then
    printf "Error: No release type specified. Please provide a release type (major, minor, patch).\n\n"
    exit 1
  fi

  if [ "$release_type" != "major" ] && [ "$release_type" != "minor" ] && [ "$release_type" != "patch" ]; then
    printf "Error: Invalid release type specified. Please provide a valid release type (major, minor, patch). Passed: %s.\n\n" "$release_type"
    exit 1
  fi

  IFS='.' read -r major minor patch <<EOF
  $version
EOF

  case $release_type in
    major)
      major=$((major + 1))
      minor=0
      patch=0
      ;;
    minor)
      minor=$((minor + 1))
      patch=0
      ;;
    patch)
      patch=$((patch + 1))
      ;;
  esac

  # Trim any leading or trailing whitespace from the version components
  major=$(echo "$major" | xargs)
  minor=$(echo "$minor" | xargs)
  patch=$(echo "$patch" | xargs)


  echo "$major.$minor.$patch"
}