cargo +nightly build --release
copy target\release\map_service.dll examples\map_service.pyd
copy target\release\map_service.dll ..\server\map_service.pyd