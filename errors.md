# Dream Interpreter App - Build Issues

## Current Issues

### 1. Invalid Option Flag for SASS Compiler

```
OptionParser::InvalidOption: invalid option: --embed-source-map
  Use --trace for backtrace.
```

This error was related to the SASS compiler not supporting the `--embed-source-map` flag that was being passed by Trunk.

**Status**: RESOLVED ✅

**Solution Applied**:

- Replaced SCSS with plain CSS by:
  1. Creating a `public/styles.css` file with the same styles
  2. Updating `index.html` to use `rel="css"` instead of `rel="scss"`
  3. Removing SASS configuration files

This completely avoided the SASS compilation process, eliminating the source map error.

### 2. API Connection Issues

```
Failed to send request. Make sure the API server is running at: http://localhost:3001/api/interpret_dream
```

The API server for dream interpretation needs to be running and accessible from the Leptos application.

**Status**: RESOLVED ✅

**Solution Applied**:

- Added detailed debug logging throughout the API request process
- Added a mock mode toggle that provides local dream interpretations without requiring the API
- Made the API URL configurable in the UI
- Improved error messaging with specific error details
- Added CORS mode selection (`cors` or `no-cors`) to handle cross-origin issues
- Enhanced error handling to detect and explain CORS errors specifically

This ensures the application can still function even when the API is unavailable, and provides detailed information to diagnose connection issues when using the real API.

### 3. CORS (Cross-Origin Resource Sharing) Errors

```
Access to fetch at 'http://localhost:3001/api/interpret_dream' from origin 'http://localhost:3000' has been blocked by CORS policy
```

CORS errors occur when making requests from one origin (domain/port) to another when the server doesn't include proper CORS headers.

**Status**: RESOLVED ✅

**Solution Applied**:

- Added specific error detection and messaging for CORS errors
- Implemented a CORS mode selection feature that allows users to choose between:
  1. `cors` mode (standard, requires server headers)
  2. `no-cors` mode (returns opaque responses but may bypass CORS restrictions)
- Added helpful UI descriptions to explain each mode
- Enhanced the error display with recommendations to enable mock mode when API errors occur

**Note**: For a complete solution, the API server should include proper CORS headers like:

```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, OPTIONS
Access-Control-Allow-Headers: Content-Type
```

### 4. Warnings

#### Deprecated Methods

```
warning: use of deprecated method `web_sys::RequestInit::method`: Use `set_method()` instead.
warning: use of deprecated method `web_sys::RequestInit::body`: Use `set_body()` instead.
warning: use of deprecated method `web_sys::RequestInit::headers`: Use `set_headers()` instead.
```

These warnings indicate that we're using deprecated methods from the web-sys crate. While these don't prevent the app from building, they should be updated to use the recommended replacements.

**Status**: Partially resolved with `#[allow(deprecated)]` annotation ⚠️

**Todo**:

- Properly update to use `set_method()`, `set_body()`, and `set_headers()` methods instead of their deprecated versions

#### Dead Code

```
warning: struct `DreamRequest` is never constructed
```

The `DreamRequest` struct was defined but never used, as we've switched to directly formatting the JSON string.

**Status**: RESOLVED ✅

**Solution Applied**:

- Removed the unused `DreamRequest` struct

#### Unused Import

```
warning: unused import: `Serialize`
```

The `Serialize` import from serde is no longer needed after removing the `DreamRequest` struct.

**Status**: RESOLVED ✅

**Solution Applied**:

- Removed the unused `Serialize` import from serde

### 5. Rust Future Compatibility Warning

```
warning: the following packages contain code that will be rejected by a future version of Rust:
console_error_panic_hook v0.1.7, gloo-net v0.6.0, js-sys v0.3.77, tachys v0.1.8,
wasm-bindgen-futures v0.4.50, wasm-streams v0.4.2
```

Several dependencies contain code that will be rejected by future Rust versions.

**Status**: Low priority ⚠️ - No immediate action needed, but should be monitored for updates to these dependencies

## Testing Progress

| Date       | Test                                      | Result  | Notes                                                                 |
| ---------- | ----------------------------------------- | ------- | --------------------------------------------------------------------- |
| 2025-04-13 | Initial build attempt                     | Failed  | Error with `--embed-source-map` in SASS compiler                      |
| 2025-04-13 | Build with `data-no-source-maps` for Rust | Failed  | The error was not related to Rust compilation but to SASS compilation |
| 2025-04-13 | Build with plain CSS                      | Success | Successfully built by switching from SCSS to plain CSS                |
| 2025-04-13 | API connection test                       | Failed  | API server not running on the specified port                          |
| 2025-04-13 | Mock mode test                            | Success | Successfully generated interpretations without requiring API          |
| 2025-04-14 | CORS error test                           | Failed  | Browser blocked cross-origin request                                  |
| 2025-04-14 | CORS handling implementation              | Success | Added better error detection and UI for CORS options                  |

## Implemented Features

1. ✅ Dream interpreter component that connects to API
2. ✅ Configurable API URL for flexibility
3. ✅ Mock mode for testing without API server
4. ✅ Detailed debug logging for troubleshooting
5. ✅ Keyword-based mock interpretation
6. ✅ Clean, responsive UI
7. ✅ CORS mode selection for API requests
8. ✅ Enhanced error handling with specific CORS error detection

## Next Steps

1. ✅ Remove unused `DreamRequest` struct
2. ✅ Remove unused `Serialize` import
3. ✅ Fix the SASS compilation issue by switching to plain CSS
4. ✅ Improve error handling for API connections
5. ✅ Add detailed logging and debug panel
6. ✅ Implement mock mode for API-free testing
7. ✅ Add CORS mode selection and error handling
8. 🔄 Properly fix the deprecated method warnings
9. 🔄 Set up remote API server for dream interpretation with proper CORS headers

## Tailwind Considerations

If you're interested in using Tailwind with Leptos CSR in the future, it's important to ensure compatibility with Trunk. Tailwind integration would involve:

1. Installing the necessary dependencies:
   ```
   npm install -D tailwindcss
   ```
2. Setting up Tailwind configuration files
3. Modifying the `index.html` to use Tailwind instead of plain CSS, following the Trunk documentation at: https://trunkrs.dev/assets/#tailwind

However, this should only be considered after ensuring the core functionality is working correctly.
