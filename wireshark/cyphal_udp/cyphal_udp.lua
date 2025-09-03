--
-- This is a basic Cyphal/UDP dissector for Wireshark.
--
-- Features:
-- * Basic display of header fields
-- * Names of regulated fixed ports
--
-- Not yet implemented:
-- * Header CRC checking
-- * Transfer CRC checking
-- * Payload decoding
-- * Linking requests to responses
-- * Checking if the Cyphal/UDP header fields match the IP header
--
-- To use:
--
-- Option: 1:
-- 1. Open Wireshark
-- 2. Open the "About Wireshark" window
-- 3. Go to the "Folders" tab
-- 4. Note the path in the "Personal Lua Plugins" row
-- 5. Copy this file into that folder
-- 6. Restart Wireshark
--
-- Option 2: `wireshark -X lua_script:cyphal_udp.lua`
--

local HEADER_LEN = 24
local CYPHAL_UDP_PORT = 9382

local cyphal_udp = Proto("cyphal_udp", "Cyphal/UDP")

local pf_version = ProtoField.uint8("cyphal_udp.version", "Version")

local priority_names = {
    [0] = "Exceptional",
    [1] = "Immediate",
    [2] = "Fast",
    [3] = "High",
    [4] = "Nominal",
    [5] = "Low",
    [6] = "Slow",
    [7] = "Optional",
}

local pf_priority = ProtoField.uint8("cyphal_udp.priority", "Priority", base.DEC, priority_names)
local pf_source_node = ProtoField.uint16("cyphal_udp.source_node", "Source node ID")
local pf_destination_node = ProtoField.uint16("cyphal_udp.destination_node", "Destination node ID")

local DATA_SPEC_SERVICE_NOT_MESSAGE = 0x8000
local DATA_SPEC_REQUEST_NOT_RESPONSE = 0x4000
-- Bits available for the subject ID in a data specifier
local DATA_SPEC_SUBJECT_ID_MASK = ~DATA_SPEC_SERVICE_NOT_MESSAGE
-- Bits available for the service ID in a data specifier
local DATA_SPEC_SERVICE_ID_MASK = ~(DATA_SPEC_SERVICE_NOT_MESSAGE | DATA_SPEC_REQUEST_NOT_RESPONSE)

local pf_transfer_type = ProtoField.string("cyphal_udp.transfer_type", "Transfer type")

local fixed_subject_ids = {
    [7168] = "uavcan.time.Synchronization.1.0",
    [7509] = "uavcan.node.Heartbeat.1.0",
    [7510] = "uavcan.port.List.1.0",
    [8164] = "uavcan.pnp.cluster.Discovery.1.0",
    [8165] = "uavcan.pnp.NodeIDAllocationData.2.0",
    [8166] = "uavcan.pnp.NodeIDAllocationData.1.0",
    [8174] = "uavcan.internet.udp.OutgoingPacket.0.1",
    [8184] = "uavcan.diagnostic.Record.1.0",
}

local pf_subject_id = ProtoField.uint16("cyphal_udp.subject_id", "Subject ID", base.DEC, fixed_subject_ids, DATA_SPEC_SUBJECT_ID_MASK)

local fixed_service_ids = {
    [384] = "uavcan.register.Access.1.0",
    [385] = "uavcan.register.List.1.0",
    [390] = "uavcan.pnp.cluster.AppendEntries.1.0",
    [391] = "uavcan.pnp.cluster.RequestVote.1.0",
    [405] = "uavcan.file.GetInfo.0.2",
    [406] = "uavcan.file.List.0.2",
    [407] = "uavcan.file.Modify.1.1",
    [408] = "uavcan.file.Read.1.1",
    [409] = "uavcan.file.Write.1.1",
    [500] = "uavcan.internet.udp.HandleIncomingPacket.0.2",
    [430] = "uavcan.node.GetInfo.1.0",
    [434] = "uavcan.node.GetTransportStatistics.0.1",
    [435] = "uavcan.node.ExecuteCommand.1.3",
    [510] = "uavcan.time.GetSynchronizationMasterInfo.0.1",
}

local pf_service_id = ProtoField.uint16("cyphal_udp.service_id", "Service ID", base.DEC, fixed_service_ids, DATA_SPEC_SERVICE_ID_MASK)

local pf_transfer_id = ProtoField.uint64("cyphal_udp.transfer_id", "Transfer ID")

local END_OF_TRANFER_MASK = 1 << 31
local pf_frame_index = ProtoField.uint32("cyphal_udp.frame_index", "Frame index", base.DEC, nil, ~END_OF_TRANFER_MASK)
local pf_transfer_eot = ProtoField.bool("cyphal_udp.end_of_transfer", "End of transfer", ftypes.UINT32, nil, END_OF_TRANFER_MASK)
local pf_user_data = ProtoField.uint16("cyphal_udp.user_data", "User data", base.HEX)
local pf_header_crc = ProtoField.uint16("cyphal_udp.header_crc", "Header CRC", base.HEX)

cyphal_udp.fields = {
    pf_version,
    pf_priority,
    pf_source_node,
    pf_destination_node,
    pf_transfer_type,
    pf_subject_id,
    pf_service_id,
    pf_transfer_id,
    pf_transfer_eot,
    pf_frame_index,
    pf_user_data,
    pf_header_crc,
}


function cyphal_udp.dissector(tvbuf, pktinfo, root)
    local pktlen = tvbuf:reported_length_remaining()
    if pktlen < HEADER_LEN then
        return
    end
    pktinfo.cols.protocol:set("Cyphal/UDP")
    local tree = root:add(cyphal_udp, tvbuf:range(0, HEADER_LEN))

    tree:add(pf_version, tvbuf:range(0,1))
    tree:add(pf_priority, tvbuf:range(1,1))
    local source_node = tvbuf:range(2,2):le_uint()
    local destination_node = tvbuf:range(4,2):le_uint()
    tree:add_le(pf_source_node, tvbuf:range(2,2))
    tree:add_le(pf_destination_node, tvbuf:range(4,2))

    local data_specifier = tvbuf:range(6,2):le_uint()
    if (data_specifier & DATA_SPEC_SERVICE_NOT_MESSAGE) ~= 0 then
        local service_id = data_specifier & DATA_SPEC_SERVICE_ID_MASK
        pktinfo.cols.info:set("Service " .. service_id)
        if (data_specifier & DATA_SPEC_REQUEST_NOT_RESPONSE) ~= 0 then
            tree:add(pf_transfer_type, "Service request"):set_generated()
            pktinfo.cols.info:append(" request " .. source_node .. " → " .. destination_node)
        else
            tree:add(pf_transfer_type, "Service response"):set_generated()
            pktinfo.cols.info:append("response " .. source_node .. " → " .. destination_node)
        end
        tree:add_le(pf_service_id, tvbuf:range(6,2))
        local service_name = fixed_service_ids[service_id]
        if service_name ~= nil then
            pktinfo.cols.info:append(" (" .. service_name .. ")")
        end
    else
        tree:add(pf_transfer_type, "Message"):set_generated()
        tree:add_le(pf_subject_id, tvbuf:range(6,2))
        local subject_id = tvbuf:range(6,2):le_uint()
        pktinfo.cols.info:set("Message " .. subject_id .. " from " .. source_node)
        local subject_name = fixed_subject_ids[subject_id]
        if subject_name ~= nil then
            pktinfo.cols.info:append(" (" .. subject_name .. ")")
        end
    end

    tree:add_le(pf_transfer_id, tvbuf:range(8,8))
    tree:add_le(pf_transfer_eot, tvbuf:range(16,4))
    tree:add_le(pf_frame_index, tvbuf:range(16,4))
    tree:add_le(pf_user_data, tvbuf:range(20,2))
    tree:add(pf_header_crc, tvbuf:range(22,2))

    return HEADER_LEN
end

-- Run on all UDP packets going to the correct port
DissectorTable.get("udp.port"):add(CYPHAL_UDP_PORT, cyphal_udp)
