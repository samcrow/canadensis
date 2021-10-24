#!/usr/bin/env python3

import pyuavcan
import pyuavcan.transport.udp
import  pyuavcan.transport.commons.crc

major_general_song = b"I am the very model of a modern Major-Gineral, \
I've information vegetable, animal, and mineral, \
I know the kings of England, and I quote the fights historical \
From Marathon to Waterloo, in order categorical; \
I'm very well acquainted, too, with matters mathematical, \
I understand equations, both the simple and quadratical, \
About binomial theorem I'm teeming with a lot o' news, \
With many cheerful facts about the square of the hypotenuse. \
I'm very good at integral and differential calculus; \
I know the scientific names of beings animalculous: \
In short, in matters vegetable, animal, and mineral, \
I am the very model of a modern Major-Gineral. \
I know our mythic history, King Arthur's and Sir Caradoc's; \
I answer hard acrostics, I've a pretty taste for paradox, \
I quote in elegiacs all the crimes of Heliogabalus, \
In conics I can floor peculiarities parabolous; \
I can tell undoubted Raphaels from Gerard Dows and Zoffanies, \
I know the croaking chorus from The Frogs of Aristophanes! \
Then I can hum a fugue of which I've heard the music's din afore, \
And whistle all the airs from that infernal nonsense Pinafore. \
Then I can write a washing bill in Babylonic cuneiform, \
And tell you ev'ry detail of Caractacus's uniform: \
In short, in matters vegetable, animal, and mineral, \
I am the very model of a modern Major-Gineral. \
In fact, when I know what is meant by \"mamelon\" and \"ravelin\", \
When I can tell at sight a Mauser rifle from a javelin, \
When such affairs as sorties and surprises I'm more wary at, \
And when I know precisely what is meant by \"commissariat\", \
When I have learnt what progress has been made in modern gunnery, \
When I know more of tactics than a novice in a nunnery - \
In short, when I've a smattering of elemental strategy - \
You'll say a better Major-General has never sat a gee. \
For my military knowledge, though I'm plucky and adventury, \
Has only been brought down to the beginning of the century; \
But still, in matters vegetable, animal, and mineral, \
I am the very model of a modern Major-Gineral."

shorter_payload = b"I am the very model of a modern Major-Gineral, \
I've information vegetable, animal, and mineral, \
I know the kings of England, and I quote the fights historical \
From Marathon to Waterloo, in order categorical;"

# Make a payload compatible with the uavcan.metatransport.ethernet.Frame.0.1 format
string_length = len(major_general_song)
payload = bytes([0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5]) +\
          bytes([0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5]) +\
          bytes([0x00, 0x08]) +\
          bytes([string_length & 0xff, (string_length >> 8) & 0xff]) +\
          major_general_song

# Calculate CRC
crc = pyuavcan.transport.commons.crc.CRC32C()
crc.add(payload)
print("CRC of {} bytes of payload: {:#08x}".format(len(payload), crc.value))

# Split into 1000 bytes per frame
# TODO: Does this really do anything? It looks like the transport puts this back together and then fragments it again.
payload_per_frame = 1000
fragmented = [payload[i:i + payload_per_frame] for i in range(0, len(payload), payload_per_frame)]

tr_0 = pyuavcan.transport.udp.UDPTransport('127.0.0.5')
pm = pyuavcan.transport.PayloadMetadata(4096)
ds = pyuavcan.transport.MessageDataSpecifier(73)
pub = tr_0.get_output_session(pyuavcan.transport.OutputSessionSpecifier(ds, None), pm)
transfer_id = 0

await_ = tr_0.loop.run_until_complete

await_(pub.send(pyuavcan.transport.Transfer(pyuavcan.transport.Timestamp.now(),
                                           pyuavcan.transport.Priority.LOW,
                                           transfer_id,
                                           fragmented_payload=fragmented),

               tr_0.loop.time() + 1.0))