-- atx_status.lua
-- Queries the current ATX LED state from the JetKVM ATX Power Control extension.
-- Returns a JSON object with "power" (bool) and "hdd" (bool) fields indicating
-- whether the power LED and HDD activity LED are currently lit.
--
-- Usage:
--   jetkvm_control atx_status.lua

print("Querying ATX state...")
local result = send_rpc("getATXState", '{}')
print("ATX state: " .. result)
