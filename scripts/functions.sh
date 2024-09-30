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
  # shellcheck disable=SC3043
  local pre_release_type="$3"

  if [ -z "$release_type" ]; then
    echo "Error: No release type specified. Please provide a release type (major, minor, patch)."
    exit 1
  fi

  if [ "$release_type" != "major" ] && [ "$release_type" != "minor" ] && [ "$release_type" != "patch" ]; then
    echo "Error: Invalid release type specified. Please provide a valid release type (major, minor, patch)."
    exit 1
  fi

  # Split version and pre-release identifier
  read -r core_version pre_release <<EOF
$(echo "$version" | tr '-' ' ')
EOF

  # Split core version into major, minor, and patch
  read -r major minor patch <<EOF
$(echo "$core_version" | tr '.' ' ')
EOF

  case $release_type in
    major)
      major=$((major + 1))
      minor=0
      patch=0
      pre_release=""
      ;;
    minor)
      minor=$((minor + 1))
      patch=0
      pre_release=""
      ;;
    patch)
      patch=$((patch + 1))
      pre_release=""
      ;;
  esac

  if [ -n "$pre_release_type" ]; then
    if [ -z "$pre_release" ]; then
      pre_release="$pre_release_type.1"
    else
      read -r current_pre_release_type current_pre_release_version <<EOF
$(echo "$pre_release" | tr '.' ' ')
EOF
      if [ "$current_pre_release_type" = "$pre_release_type" ]; then
        current_pre_release_version=$((current_pre_release_version + 1))
        pre_release="$pre_release_type.$current_pre_release_version"
      else
        pre_release="$pre_release_type.1"
      fi
    fi
  fi

  # Trim any leading or trailing whitespace from the version components
  major=$(echo "$major" | xargs)
  minor=$(echo "$minor" | xargs)
  patch=$(echo "$patch" | xargs)

  if [ -z "$pre_release" ]; then
    echo "$major.$minor.$patch"
  else
    echo "$major.$minor.$patch-$pre_release"
  fi
}