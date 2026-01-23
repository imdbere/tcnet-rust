TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# INTRODUCTION

TCNet is originally designed by developers from the entertainment industry to create an open communication protocol between devices or software to share real-time Time Code and Meta Data.

The protocol is open and free to be used and everyone can contribute.

# COMMUNICATION VIA NODES

TCNet is designed to have virtually unlimited amount of nodes that can participate. Each node is identified by its own unique MAC address and can have one of three roles: Auto, Master, Slave or Repeater. For example: A Master generates TCNet Time Code packets and sends these to all Slaves in network, A Slave only receives TCNet Metadata and Timing packets. A Repeater is capable of receiving AND sending TCNet Metadata and Timing Packets. No matter what role a node is, it is capable to send and receive TCNet Control Message packets.

TCNet Opt-IN packets are sent by a node, every 1000 milliseconds to establish and keep participation in a TCNet network.

Each node finds and populates other nodes this way and holds a active population list of all nodes and its functions, listener port and timer.

When a node disconnects or disappears from a TCNet network, it should be automatically deleted from the population list.

# NETWORK PORTS

TCNet communicates via the UDP protocol. The following ports are used:

## Broadcast ports:

60000 - Used for broadcasted messages like Opt-IN and Opt-OUT messages

60000 - Used for Application Specific Data (Non public data shared between applications)

60001 - Used for broadcasting TCNet Time Packets

## Unicast ports:

65023-65535 - Used for unicast messages. (Default is 65023)

# NETWORK PARTICIPATION

To join a TCNet network the following steps need to be taken:

## First step:

Create an internal timer that runs from 0-999999 Microseconds (This can also be done by using computers internal clock and take Microseconds of each second cycle)

## Second step:

Open a listener on port 60000,60001,60002 to receive TCNet broadcast packets.

## Third step:

Send a TCNet GW Opt-IN package every 1000 milliseconds, containing basic information and functionality of the node. (See: OPT-IN/OPT-OUT MESSAGES)

## Fourth step:

Wait for incoming Opt-IN messages and keep track of all nodes in a list. Each Node tells what port to use to communicate.

## Additional step:

Perform a time sync between all discovered nodes. (See: SYNC MESSAGES)

After joining a TCNet network, depending on your node's role, you can send and receive information.

The basic rule is that only a Master or Repeater can send data and that a Slave or Repeater only can request data.

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TC

# OPT-IN/OPT-OUT MESSAGES

The following Opt-In/Opt-Out message types are defined in this document:

002 – TCNet OPT-IN Packet (Broadcast on port 60000)
003 – TCNet OPT-OUT Packet (Broadcast on port 60000)

# STATUS MESSAGES

Broadcast of Realtime Status messages:

005 – TCNet Status Packet (Broadcast on port 60000, Unicast to all slaves)

# SYNC MESSAGES

The following Sync message types are defined in this document:

010 – TCNet Time Sync Packet (Unicast on port Target-Node-Port)

# NOTIFICATION MESSAGES

Control messages are special messages that allow remote control TCNet nodes. (Full documentation upon request)
The following control message types are defined in this document:

013 – TCNet Error Notification Packet (Unicast on port Target-Node-Port)
020 – TCNet Request Packet (Unicast on port Target-Node-Port)
030 – TCNet Application Specific Data Packet (Broadcast on port 60001, Unicast on Target-Node-Port)

# CONTROL MESSAGES

Control messages are special messages that allow remote control TCNet nodes. (Full documentation upon request)
The following control message types are defined in this document:

101 – TCNet Control Messages (Unicast on port Target-Node-Port)
128 – TCNet Text Data (Broadcast on port 60000 or Unicast on port Target-Node-Port)
132 – TCNet Keyboard Data (Broadcast on port 60000 or Unicast on port Target-Node-Port)

# DATA PACKETS

Data message types are messages containing data such as metadata, timing data, waveform data, cues etc.
The following data message types are defined in this document:

200 – TCNet Data Packet – Metrics Data (Unicast on port Target-Node-Port) (Type 2)
200 – TCNet Data Packet – Metadata (Unicast on port Target-Node-Port) (Type 4)
200 – TCNet Data Packet – Beat Grid Info (Unicast on port Target-Node-Port) (Type 8)
200 – TCNet Data Packet – Cue Data Info (Unicast on port Target-Node-Port) (Type 12)
200 – TCNet Data Packet – Small Wave Form (Unicast on port Target-Node-Port) (Type 16)
200 – TCNet Data Packet – Big Wave Form (Unicast on port Target-Node-Port) (Type 32)
200 – TCNet Data Packet – Mixer Data (Unicast to all slaves) (Type 150)

# FILE PACKETS

File packet types are packets containing data such as images and audio files.
The following data message types are defined in this document:

204 – TCNet Data File Packet – Low Res Artwork Image (Unicast on port Target-Node-Port) (Type 128)

# APPLICATION SPECIFIC DATA PACKETS

Application Specific Data packet types are packets containing data exchanged between applications.
The following data message types are defined in this document:

213 – TCNet Application Specific Data (Broadcast on port 60000, Unicast on Target-Node-Port)

# TIMING PACKETS

Time Packets are time critical and updated at high rates.

254 – TCNet Time Packet (Broadcast on port 60001, Unicast on Target-Node-Port)

# NODE OPTIONS

When a node opts in on a TCNet network, the communication flags can be set in this byte. If you need to set more flags than one, just sum the flags (Flag 1+ Flag 2+ Flag 8 = 11)
The following flags are available:

1 – NEED AUTHENTICATION (Authentication for extended communication needed)
2 – SUPPORTS TCNCM (Listens to TCNet Control Messages)
4 – SUPPORTS TCNASDP (Listens to TCNet Application Specific Data Packet)
8 – DNO (Do not disturb/Sleeping. Node will request data itself if needed to avoid traffic)

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TC™

# FLAME VERSIONS

To make sure TCNet is backwards compatible, a flame number is used for each addition or change. To make your applications backwards compatible with older versions, always check for the protocol version of incoming packets.

# INFORMATION

For more background information or documentation, please don't hesitate to make inquiries to dev@eiglive.com.

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
3

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Opt-IN Packet

Functionality: Present and keep alive a node into a TCNet network.
Type: Broadcast and Unicast
Port: UDP(60000) and destination node's port
Size: 68
Behavior: Broadcast every 1000ms

|  PARAMETERS | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 6 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 2 | Type 2: TCNet OPT-IN | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Node Count | 24 | 2 | 0-65535 (LITTLE ENDIAN) | Amount of Registered Nodes | V3-1  |
|  Node Listener Port | 26 | 2 | 65023-65535 (LITTLE ENDIAN) | Listener Port for Unicast Messages | V3-1  |
|  Uptime | 28 | 2 | 0-43199 (LITTLE ENDIAN) | Uptime of Node in SEC | V3-2  |
|  RESERVED | 30 | 2 |  | RESERVED | V3-2  |
|  Vendor Name | 32 | 16 | ASCII TEXT | Vendor | V3-2  |
|  Application/Device Name | 48 | 16 | ASCII TEXT | Application / Device Name | V3-2  |
|  Application/Device Major Version | 64 | 1 | 0-255 | Application/Device Major Version | V3-2  |
|  Application/Device Minor Version | 65 | 1 | 0-255 | Application/Device Minor Version | V3-2  |
|  Application/Device Bug Version | 66 | 1 | 0-255 | Application/Device Minor Version | V3-2  |
|  RESERVED | 67 | 1 |  | RESERVED | V3-2  |

* See details below:

# TCNet Opt-IN Packet – Details

Node ID: Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version: Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header: TCNet Protocol Header (Must be "TCN")
Message Type: Message type of packet. - Value=2
Node Name: GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ: Sequence number of packet. (See Sequence number)
Node Type: Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options: Node options: See Node Options
Timestamp: Time stamp in microseconds that is used to calculate network latency.
Node Count: Number of nodes registered by system
Node Listener Port: Listener port of node (Used to receive unicast messages)
Uptime: Up time of Node in seconds. (1) Must Roll over / Reset every 12 hours.
Vendor Name: Name of Vendor of Node
Application/Device Name: Name of Application/Device (Node)
Major Version: Major Version of Node
Minor Version: Minor Version of Node
Bug Version: Bug Version of Node

# TCNet Opt-IN Packet – Usage

Inorder to correctly implement the Opt-In usage, the following steps are needed.

Step 1:
Create a Opt-IN packet and make sure your Node Listeners Port value is correct.

Step 2:
Broadcast every 1000ms a Opt-IN packet to port 60000

Step 3:
Unicast every 1000ms a Opt-IN packet to each discovered node, targeting that node's port. This ensures that when a node doesn't receive broadcast messages, it still can discover your node.

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Opt-OUT Packet

Functionality: Notifies other nodes that node leaves network.
Type: Broadcast and Unicast
Port: UDP(60000) and destination node's port
Size: 28
Behavior: Broadcast and Unicast once when leaving network

|  PARAMETERS | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 3 | Type 3: TCNet OPT-OUT | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Node Count | 24 | 2 | 0-65535 (LITTLE ENDIAN) | Amount of Registered Nodes | V3-1  |
|  Node Listener Port | 26 | 2 | 65023-65535 (LITTLE ENDIAN) | Listener Port for Unicast Messages | V3-1  |

* See details below:

# TCNet Opt-OUT Packet - Details

Node ID: Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version: Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header: TCNet Protocol Header (Must be "TCN")
Message Type: Message type of packet. Value=3
Node Name: GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ: Sequence number of packet. (See Sequence number)
Node Type: Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options: Node options: See Node Options
Timestamp: Timestamp in microseconds that is used to calculate network latency.
Node Count: Number of nodes registered by system
Node Listener Port: Listener port of node (Used to receive unicast messages)

# TIP:

In case of a disconnect of a Master Node in the network, the next master is chosen by looking at all Nodes running as Node Type 1 (Auto Master).

The node that has the highest Uptime including Timestamp becomes the new master. This node changes its type to 2 (Master) and starts its services as such.

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

# TCNet Status Packet

Functionality Status PACKET of current settings on node.
Type Broadcast
Port UDP(60000)
Size 300
Behavior Broadcast every 1000ms

|  PARAMETERS | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 6 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 5 | Type 5: TCNet STATUS | V3-3  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Node Count | 24 | 2 | 0-65535 (LITTLE ENDIAN) | Amount of Registered Nodes | V3-3  |
|  Node Listener Port | 26 | 2 | 65023-65535 (LITTLE ENDIAN) | Listener Port for Unicast Messages | V3-3  |
|  RESERVED | 28 | 6 |  | RESERVED | V3-3  |
|  Layer 1 Source | 34 | 1 | 0-255 | Layer 1 Source | V3-3  |
|  Layer 2 Source | 35 | 1 | 0-255 | Layer 2 Source | V3-3  |
|  Layer 3 Source | 36 | 1 | 0-255 | Layer 3 Source | V3-3  |
|  Layer 4 Source | 37 | 1 | 0-255 | Layer 4 Source | V3-3  |
|  Layer A Source | 38 | 1 | 0-255 | Layer A Source | V3-3  |
|  Layer B Source | 39 | 1 | 0-255 | Layer B Source | V3-3  |
|  Layer M Source | 40 | 1 | 0-255 | Layer M Source | V3-3  |
|  Layer C Source | 41 | 1 | 0-255 | Layer C Source | V3-3  |
|  Layer 1 Status | 42 | 1 | 0-255 | Layer 1 Status | V3-3  |
|  Layer 2 Status | 43 | 1 | 0-255 | Layer 2 Status | V3-3  |
|  Layer 3 Status | 44 | 1 | 0-255 | Layer 3 Status | V3-3  |
|  Layer 4 Status | 45 | 1 | 0-255 | Layer 4 Status | V3-3  |
|  Layer A Status | 46 | 1 | 0-255 | Layer A Status | V3-3  |
|  Layer B Status | 47 | 1 | 0-255 | Layer B Status | V3-3  |
|  Layer M Status | 48 | 1 | 0-255 | Layer M Status | V3-3  |
|  Layer C Status | 49 | 1 | 0-255 | Layer C Status | V3-3  |
|  Layer 1 Track ID | 50 | 4 | 0-FFFFFFF* (LITTLE ENDIAN) | Assigned Track ID for Layer 1 | V3-3  |
|  Layer 2 Track ID | 54 | 4 | 0-FFFFFFF* (LITTLE ENDIAN) | Assigned Track ID for Layer 2 | V3-3  |
|  Layer 3 Track ID | 58 | 4 | 0-FFFFFFF* (LITTLE ENDIAN) | Assigned Track ID for Layer 3 | V3-3  |
|  Layer 4 Track ID | 62 | 4 | 0-FFFFFFF* (LITTLE ENDIAN) | Assigned Track ID for Layer 4 | V3-3  |
|  Layer A Track ID | 66 | 4 | 0-FFFFFFF* (LITTLE ENDIAN) | Assigned Track ID for Layer A | V3-3  |
|  Layer B Track ID | 70 | 4 | 0-FFFFFFF* (LITTLE ENDIAN) | Assigned Track ID for Layer B | V3-3  |
|  Layer M Track ID | 74 | 4 | 0-FFFFFFF* (LITTLE ENDIAN) | Assigned Track ID for Layer M | V3-3  |
|  Layer C Track ID | 78 | 4 | 0-FFFFFFF* (LITTLE ENDIAN) | Assigned Track ID for Layer C | V3-3  |
|  RESERVED | 82 | 1 |  | RESERVED | V3-3  |
|  SMPTE Mode | 83 | 1 | 0-255 | SMPTE Mode | V3-3  |
|  Auto Master Mode | 84 | 1 | 0-255 | RESERVED | V3-3  |

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

|   | RESERVED | 85 | 15 |  | RESERVED | V3-3  |
| --- | --- | --- | --- | --- | --- | --- |
|   |  RESERVED (APP SPECIFIC) | 100 | 72 |  | APP SPECIFIC | V3-3  |
|   |  Layer 1 Name | 172 | 16 |  | Name of Layer 1 | V3-3-2  |
|   |  Layer 2 Name | 188 | 16 |  | Name of Layer 2 | V3-3-2  |
|   |  Layer 3 Name | 204 | 16 |  | Name of Layer 3 | V3-3-2  |
|   |  Layer 4 Name | 220 | 16 |  | Name of Layer 4 | V3-3-2  |
|   |  Layer A Name | 236 | 16 |  | Name of Layer 5 | V3-3-2  |
|   |  Layer B Name | 252 | 16 |  | Name of Layer 6 | V3-3-2  |
|   |  Layer M Name | 268 | 16 |  | Name of Layer 7 | V3-3-2  |
|   |  Layer C Name | 284 | 16 |  | Name of Layer 8 | V3-3-2  |

* See details below:

# TCNet Status Packet – Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.

Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00

Header TCNet Protocol Header (Must be "TCN")

Message Type Message type of packet: STATUS - Value=5

Node Name GW Code of software/machine/source that sends packet. (8 Characters)

Example: ABCDEFGH

SEQ Sequence number of packet. (See Sequence number)

Node Type Node Type

Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater

Node Options Node options: See Node Options

Timestamp Time stamp in microseconds that is used to calculate network latency.

Node Count Number of nodes registered by system

Node Listener Port Listener port of node (Used to receive unicast messages)

Layer Source Source number of layer

Layer Status Play head status of layer

Example: 0=IDLE, 3,=PLAYING, 4=LOOPING, 5=PAUSED, 6=STOPPED, 7=CUE BUTTON DOWN, 8=PLATTER DOWN, 9=FFWD, 10=FFRV, 11=HOLD

Layer Track ID Track ID of track loaded on layer

SMPTE Mode SMPTE Mode set on node

Values: 24=24FPS, 25=25FPS, 29=29.7FPS, 30=30FPS

Auto Master Mode Auto Master mode on node (0=Disabled, 1=HTP Master, 2=Link Master)

App Specific Application Specific Data

Layer Name Name of Layer

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Time Sync Packet

Functionality Send and Receive Time Sync Data.
Type Unicast
Port UDP(Target-Node-Port)
Size 32
Behavior Response Required

|  PARAMETERS | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 10 | Type 10: TCNet Time Sync Packet | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  STEP | 24 | 1 | 0-3 | Step No | V3-1  |
|  RESERVED | 25 | 1 |  | RESERVED |   |
|  Node Listener Port | 26 | 2 | 65023-65535 (LITTLE ENDIAN) | Listener Port for Unicast Messages | V3-2  |
|  Remote Timestamp | 28 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp of Remote Node | V3-2  |

* See details below:

# TCNet Time Sync Packet - Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header TCNet Protocol Header (Must be "TCN")
Message Type Message type of packet. - Value=10
Node Name GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ Sequence number of packet. (See Sequence number)
Node Type Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options Node options: See Node Options
Timestamp Time stamp in microseconds that is used to calculate network latency.
Step Current step in process (0=Initialize, 1=Response)
Node Listener Port Listener port of node (Used to receive unicast messages)
Remote Timestamp Time stamp send by remote node in Sync Message

# TCNet Time Sync Packet - Usage

Step 1:
Initializer send a TCNet Time Sync Message to remote node with Timestamp=Current timer in microseconds and STEP number=0

Step 2:
Remote node receives message and sends message back with Timestamp=Remote node's current timer in microseconds, STEP number=1 and Remote Timestamp=Initializer's original timestamp

Step 3:
Initializer received message back and calculates remote node's current time by:
Delay = { Current timer - Remote timestamp } /2 }
Time of remote node = Timestamp + Delay

Optional:
In order to get a more accurate timing, you can initialize the routine again and calculate more accurate by:
Delay 1 = { Current timer - Remote timestamp } /2 }
Delay 2 = { Current timer - Remote timestamp } /2 }
Time of remote node = Timestamp + ((Delay1+Delay2) / 2)

Note:
To keep track of this time, for each remote node, an internal timer should be created to keep track of current time of node.

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Error / Notification

Functionality: Notifies that a request is not handled
Type: Unicast
Port: UDP(Target-Node-Port)
Size: 30
Behavior: Send when a request is not handled or caused an error or for notifications, this message is sent back to notify requesting node.

|  PARAMETERS | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 13 | Type 13: TCNet Error Notification | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Datatype | 24 | 1 | 0-FF | Data type of Request | V3-1  |
|  Layer ID | 25 | 1 | 0-FF | Layer ID of original request | V3-1  |
|  Code | 26 | 2 | (LITTLE ENDIAN) | Returned Code | V3-1  |
|  Message Type | 28 | 2 | (LITTLE ENDIAN) | Message type of Request | V3-1  |

* See details below:

# TCNet Error / Notification - Details

Node ID: Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version: Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header: TCNet Protocol Header (Must be "TCN")
Message Type: Message type of packet: - Value=13
Node Name: GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ: Sequence number of packet. (See Sequence number)
Node Type: Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options: Node options: See Node Options
Timestamp: Time stamp in microseconds that is used to calculate network latency.
Data type: Data type of failed request
Layer ID: Layer ID of original request. If request was not targeted for specific layer, this value = 0
Code: Error / Notification Code. The following protocol codes are defined:
001 – Request Unknown (An unknown request is made)
013 – Request Not Possible/Featured (A request is recognized but can't be handled by node)
014 – Request Data = EMPTY (When a request is made for data and data is empty, this could be used to notify requesting node that there is nothing to send.
255 – Request Response: OK
Message Type: Request ID (Message Type)

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Request Packet

Functionality: Request Data from other Node
Type: Unicast
Port: UDP(Target-Node-Port)
Size: 26
Behavior: Request is sent to a master or repeater node. As result the node will send back a packet containing small wave data or a request error message.

|  PARAMETERS | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 20 | Type 20: TCNet Request Packet | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Data Type | 24 | 1 | 0-255 | Data Type | V3-1  |
|  Layer | 25 | 1 | 0-255 | Layer where data belongs to | V3-1  |

* See details below:

# TCNet Request Packet - Details

Node ID: Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version: Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header: TCNet Protocol Header (Must be "TCN")
Message Type: Message type of packet: - Value=20
Node Name: GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ: Sequence number of packet. (See Sequence number)
Node Type: Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options: Node options: See Node Options
Timestamp: Time stamp in microseconds that is used to calculate network latency.
Data Type: Data Type to request
Layer: Layer where Data is requested for

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Control Packet

Functionality Send and Receive Control Packets to control nodes remotely.

Type Unicast

Port UDP(Target-Node-Port)

Size 42 + Datasize

Behavior Response Required

|  PARAMETERS | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 101 | Type 101: TCNet Control | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  STEP | 24 | 1 | 0-1 | Step No | V3-1  |
|  RESERVED | 25 | 1 |  | RESERVED |   |
|  Data Size | 26 | 4 | (LITTLE ENDIAN) | Total Data Size | V3-2  |
|  RESERVED | 30 | 12 |  | RESERVED |   |
|  Control Path | 42 | Data Size | ASCII TEXT | String with Control Path | V3-2  |

* See details below:

# TCNet Control Packet – Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.

Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00

Header TCNet Protocol Header (Must be "TCN")

Message Type Message type of packet. - Value=101

Node Name GW Code of software/machine/source that sends packet. (8 Characters)

Example: ABCDEFGH

SEQ Sequence number of packet. (See Sequence number)

Node Type Node Type

Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater

Node Options Node options: See Node Options

Timestamp Timestamp in microseconds that is used to calculate network latency.

Step Current step in process (0=Initialize, 1=Response)

Control Path String with Control Path, examples:

To stop a layer remotely: layer/1/state=6; (6=stop)

To set layer A source layer 1: layer/5/source=1;

To set layer M source layer A: layer/7/source=5;

To set state to "play" on layer 2 and force a resync on layer 2: layer/2/state=3; layer/2/resync;

As control paths differ per application, contact your software vendor to obtain correct control path's.

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TC™

# TCNet Text Data Packet

Functionality Send and Receive Text Data Packets to control nodes remotely.
Type Broadcast/Unicast
Port UDP(6000 or Target-Node-Port)
Size 42 + Data size
Behavior Response Required

|  PARAMETERS | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 128 | Type 128: TCNet Text Data | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  STEP | 24 | 1 | 0-1 | Step No | V3-1  |
|  RESERVED | 25 | 1 |  | RESERVED |   |
|  Data Size | 26 | 4 | (LITTLE ENDIAN) | Total Data Size | V3-2  |
|  RESERVED | 30 | 12 |  | RESERVED |   |
|  Text Data | 42 | Data Size | ASCII TEXT | String Text Data | V3-2  |

* See details below:

# TCNet Text Data Packet – Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header TCNet Protocol Header (Must be "TCN")
Message Type Message type of packet. - Value=128
Node Name GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ Sequence number of packet. (See Sequence number)
Node Type Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options Node options: See Node Options
Timestamp Time stamp in microseconds that is used to calculate network latency.
Step Current step in process (0=Initialize, 1=Response)
Text Data Raw text data string

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Keyboard Data Packet

Functionality Send and Receive Realtime Keyboard Data Packets to control nodes remotely.
Type Broadcast/Unicast
Port UDP(6000 or Target-Node-Port)
Size 44
Behavior Response Required

|  PARAMETERS | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 132 | Type 132: TCNet Keyboard Data | V3-2  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  RESERVED | 24 | 1 |  | RESERVED | V3-2  |
|  RESERVED | 25 | 1 |  | RESERVED | V3-2  |
|  Data Size | 26 | 4 | (LITTLE ENDIAN) | Total Data Size | V3-2  |
|  RESERVED | 30 | 12 |  | RESERVED | V3-2  |
|  Keyboard Data | 42 | 2 | HEX ASCII Code | Keyboard Data | V3-2  |

* See details below:

# TCNet Keyboard Data Packet – Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header TCNet Protocol Header (Must be "TCN")
Message Type Message type of packet. - Value=132
Node Name GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ Sequence number of packet. (See Sequence number)
Node Type Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options Node options: See Node Options
Timestamp Timestamp in microseconds that is used to calculate network latency.
Step Current step in process (0=Initialize, 1=Response)
Keyboard Data Raw text data string

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Data Packet – Metrics Data

Functionality Updates Metrics Data for Layer
Type Unicast
Port UDP(Target-Node-Port)
Size 122
Behavior Unicast when cache changes.

|  PARAMETER | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 200 | Type 200: TCNet Data Packet | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name / Signature | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Data Type | 24 | 1 | 2 | Datatype 2 = Metrics | V2-0  |
|  Layer ID | 25 | 1 | 0-FF* | Layer Number | V2-0  |
|  RESERVED | 26 | 1 |  | RESERVED | V2-0  |
|  Layer State | 27 | 1 | 0-FF* | Layer State | V2-0  |
|  RESERVED | 28 | 1 |  | RESERVED | V2-0  |
|  Sync Master | 29 | 1 | 0-FF* | Sync Master | V2-0  |
|  RESERVED | 30 | 1 |  | RESERVED | V2-0  |
|  Beat Marker | 31 | 1 | 0-4* | Beat Marker | V2-0  |
|  Track Length | 32 | 4 | 0-0x5265C00 (LITTLE ENDIAN) | Track Length in Milliseconds | V2-0  |
|  Current Position | 36 | 4 | 0-0x5265C00 (LITTLE ENDIAN) | Play head Position in Milliseconds | V2-0  |
|  Speed | 40 | 4 | 0-20000 (LITTLE ENDIAN) | Play head Speed | V3-2  |
|  RESERVED | 44 | 13 |  | RESERVED | V3-0  |
|  Beat Number | 57 | 4 | (LITTLE ENDIAN) | Beat Number | V3-0  |
|  RESERVED | 61-111 | 51 |  |  | V3-0  |
|  BPM | 112 | 4 | 0-0x1869F* (LITTLE ENDIAN) | BPM | V3-0  |
|  Pitch Bend | 116 | 2 | (16-BIT) 0-FFFF* (LITTLE ENDIAN) | Pitch Bend | V3-0  |
|  Track ID | 118 | 4 | 0-FFFFFFF* (LITTLE ENDIAN) | Assigned Track ID | V3-0  |

* See details next page

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
14

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Data Packet - Metrics Data - Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.

Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00

Header TCNet Protocol Header (Must be "TCN")

Message Type Message type of packet. - Value=200

Node Name GW Code of software/machine/source that sends packet. (8 Characters)

Example: ABCDEFGH

SEQ Sequence number of packet. (See Sequence number)

Node Type Node Type

Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater

Node Options Node options: See Node Options

Timestamp Time stamp in microseconds that is used to calculate network latency.

Data Type Datatype of TCNet Data Packet. (Metrics Data = 2)

Layer ID Layer number of layer sending data.

Example: LAYER1, 2=LAYER2, 3=LAYER3, 4=LAYER4, 5=LAYER A, 6=LAYER B, 7=MASTER OUT, 8=RESERVED

Layer State Play head status of layer

Example: 0=IDLE, 3=PLAYING, 4=LOOPING, 5=PAUSED, 6=STOPPED, 7=CUE BUTTON DOWN, 8=PLATTER DOWN, 9=FFWD, 10=FFRV, 11=HOLD

Sync Master Sync master status of layer. Example use of this status is to follow the current active layer and allows auto cue to this layer.

Example: 0=Slave / 1=Master

Beat Marker Beat marker status of layer - Range: 1-4

Track Length Total track length of layer in milliseconds

Example: 0-9999.9999 sec

Location Marker Play head position of layer

Example: 0-9999.9999 sec

Speed Value Play head speed on layer

Example: -0-65536 (Where 32768 = 100% speed, 0 = 0% Speed, 65536=200% speed)

Beat Number Current Beat Number

BPM Value Play head BPM speed of layer

Example: 0.01-999.99

Speed Bend Value Play head speed bend value of layer. (Used for live adjust.)

Example: 0-65536 (Where 32768 = 100% speed, 0 = 0% Speed, 65536=200% speed)

Track ID Track ID number of the track that is loaded on layer. This is usually the database ID number. (Used to reflect track selection changes)

Note:
This info can be requested from a node by sending a TCNet Request Data packet, with Datatype=2, Parameter 1=LAYER, Parameter 2=0

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Data Packet - Meta Data

Functionality Contains metadata of a layer
Type Unicast
Port UDP(Target-Node-Port)
Size 548 (May change in future FLAMES)
Behavior Unicast on update event or upon request

|  PARAMETER | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 200 | Type 200: TCNet Data Packet | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name / Signature | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Data Type | 24 | 1 | 4 | Datatype 4 = Metadata | V1-0  |
|  Layer ID | 25 | 1 | 0-FF* | Layer ID | V1-0  |
|  RESERVED | 26 | 1 |  | RESERVED | V1-0  |
|  RESERVED | 27 | 2 |  | RESERVED | V1-0  |
|  Track Artist | 29 | 128/256 | ASCII TEXT (UTF-8/16 SEE BELOW) | Track Artist Name | V1-0  |
|  Track Title | 285 | 128/256 | ASCII TEXT (UTF-8/16 SEE BELOW) | Track Title Name | V1-0  |
|  Track Key | 541 | 2 | (LITTLE ENDIAN) | Track KEY | V3-2  |
|  Track ID | 543 | 4 | 0-FFFFFFF* (LITTLE ENDIAN) | Assigned Track ID | V3-3  |

* See details below:

# TCNet Data Packet - Meta Data - Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header TCNet Protocol Header (Must be "TCN")
Message Type Message type of packet. - Value=200
Node Name GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ Sequence number of packet. (See Sequence number)
Node Type Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options Node options: See Node Options
Timestamp Timestamp in microseconds that is used to calculate network latency.
Data Type Datatype of TCNet Meta Data Packet.
Layer ID Layer number if layer sending data.
Example: LAYER1, 2=LAYER2, 3=LAYER3, 4=LAYER4, 5=LAYER A, 6=LAYER B, 7=MASTER OUT, 8=RESERVED
Track ID Track ID number of the track that is loaded on layer. This is usually the source's database ID number.
Track Artist Artist name of content loaded to layer -
Example: My Artist Name (Max 256 characters)
Important! TCNet version 1.0 - 3.4.9 uses UTF-8 (Size: 256 bytes = 256 UTF-8 characters) / TCNet verion 3.5.0 and above uses UTF-16 (Size: 256 bytes = 64 UTF-16 characters)
In order to maintain backwards compatibility, please incorporate both, Unicode standards by checking remote TCNet protocol version (receiving/sending)
Track Name Track name of content loaded to layer -
Track KEY Audio Key of track

Note:
This info can be requested from a node by sending a TCNet Request Data packet, with Datatype=4, Parameter 1=LAYER, Parameter 2=0

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TC™

# TCNet Data Packet - Beat Grid Data

Functionality Contains Beat Grid Data of layer
Type Unicast
Port UDP(Target-Node-Port)
Size 2442 (May change in future FLAMES)
Behavior Unicast upon request

|  PARAMETER | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 200 | Type 200: TCNet Data Packet | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name / Signature | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data **  |   |   |   |   |   |
|  Data Type | 24 | 1 | 8 | Datatype 8 = Beat Grid Data | V3-2  |
|  Layer ID | 25 | 1 | 1-8 | Layer Number | V3-2  |
|  Data Size | 26 | 4 | (LITTLE ENDIAN) | Total Data Size | V3-2  |
|  Total Packet | 30 | 4 | 0-FF* | Total Packets used for data | V3-2  |
|  Packet No | 34 ** | 4 | (LITTLE ENDIAN) | Packet Number | V3-2  |
|  Data Cluster Size | 38 | 4 | 2400 (LITTLE ENDIAN) | Data Cluster Size | V3-2  |
|  Beat Number | 42 + OFFSET*** | 2 | (LITTLE ENDIAN) | Beat Number | V3-2  |
|  Beat Type | 44 + OFFSET*** | 1 | (LITTLE ENDIAN) | 20 = Downbeat, 10 = Up Beat | V3-2  |
|  RESERVED | 45 + OFFSET*** | 1 |  | RESERVED | V3-2  |
|  Beat Time Stamp | 46 + OFFSET*** | 4 | (LITTLE ENDIAN) | Timestamp in MS | V3-2  |

* See details below:
** Data should be split in multiple packets where each packet has a maximum of 2400 bytes of Data (Max Packet Size = 2442)
*** OFFSET = (Beat Number * 8) - (Packet No * 2400)

# TCNet Data Packet - Beat Grid Data - Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header TCNet Protocol Header (Must be "TCN")
Message Type Message type of packet. - Value=200
Node Name GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ Sequence number of packet. (See Sequence number)
Node Type Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options Node options: See Node Options
Timestamp Time stamp in microseconds that is used to calculate network latency.
Data Type Datatype of TCNet Data Packet. (Beat Grid Data=8)
Layer ID Layer number if layer sending data.
Example: LAYER1, 2=LAYER2, 3=LAYER3, 4=LAYER4, 5=LAYER A, 6=LAYER B, 7=MASTER OUT, 8=RESERVED
Total Packets Total amount of packets for data (LITTLE ENDIAN)
Packet No Packet number of data (LITTLE ENDIAN)
Data Size Total data size. Is total of all data send, including in extra packets (LITTLE ENDIAN)
Data Cluster Size Cluster Size of data (Amount of bytes used per cluster to split up total data. - Standard value = 32000
Beat Number Beat Number (LITTLE ENDIAN)
Beat Type Beat Type (20=Down Beat, 10=Upbeat)
Beat Type Beat Timestamp in MS

Note:
This info can be requested from a node by sending a TCNet Request Data packet, with Datatype=8, Layer=The layer you request data from

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Data Packet - CUE Data

Functionality Contains Cue Data of Layer
Type Unicast
Port UDP(Target-Node-Port)
Size 456 (May change in future FLAMES)
Behavior Unicast upon request

|  PARAMETER | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 200 | Type 200: TCNet Data Packet | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name / Signature | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Data Type | 24 | 1 | 12 | Datatype 12 = Cue Data | V3-2  |
|  Layer ID | 25 | 1 | 1-8 | Layer Number | V3-2  |
|  RESERVED | 26 | 16 |  | RESERVED | V3-2  |
|  Loop IN | 42 | 4 | (LITTLE ENDIAN) | Loop IN Time | V3-2  |
|  Loop OUT | 46 | 4 | (LITTLE ENDIAN) | Loop OUT Time | V3-2  |
|  CUE 1 TYPE | 47 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 48 | 1 |  | RESERVED | V3-2  |
|  CUE 1 IN TIME | 49 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 1 OUT TIME | 53 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 57 | 1 |  |  | V3-2  |
|  CUE 1 COLOR | 58 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 61 | 8 |  |  | V3-2  |
|  CUE 2 TYPE | 69 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 70 | 1 |  | RESERVED | V3-2  |
|  CUE 2 IN TIME | 71 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 2 OUT TIME | 75 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 79 | 1 |  |  | V3-2  |
|  CUE 2 COLOR | 80 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 83 | 8 |  |  | V3-2  |
|  CUE 3 TYPE | 91 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 92 | 1 |  | RESERVED | V3-2  |
|  CUE 3 IN TIME | 93 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 3 OUT TIME | 97 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 101 | 1 |  |  | V3-2  |
|  CUE 3 COLOR | 102 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 105 | 8 |  |  | V3-2  |

* See details below:

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
18

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

|  CUE 4 TYPE | 113 | 1 |  | Cue Type | V3-2  |
| --- | --- | --- | --- | --- | --- |
|  RESERVED | 114 | 1 |  | RESERVED | V3-2  |
|  CUE 4 IN TIME | 115 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 4 OUT TIME | 119 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 123 | 1 |  |  | V3-2  |
|  CUE 4 COLOR | 124 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 127 | 8 |  |  | V3-2  |
|  CUE 5 TYPE | 135 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 136 | 1 |  | RESERVED | V3-2  |
|  CUE 5 IN TIME | 137 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 5 OUT TIME | 141 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 145 | 1 |  |  | V3-2  |
|  CUE 5 COLOR | 146 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 149 | 8 |  |  | V3-2  |
|  CUE 6 TYPE | 157 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 158 | 1 |  | RESERVED | V3-2  |
|  CUE 6 IN TIME | 159 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 6 OUT TIME | 163 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 167 | 1 |  |  | V3-2  |
|  CUE 6 COLOR | 168 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 171 | 8 |  |  | V3-2  |
|  CUE 7 TYPE | 179 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 180 | 1 |  | RESERVED | V3-2  |
|  CUE 7 IN TIME | 181 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 7 OUT TIME | 185 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 189 | 1 |  |  | V3-2  |
|  CUE 7 COLOR | 190 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 193 | 8 |  |  | V3-2  |
|  CUE 8 TYPE | 201 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 202 | 1 |  | RESERVED | V3-2  |
|  CUE 8 IN TIME | 203 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 8 OUT TIME | 207 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 211 | 1 |  |  | V3-2  |
|  CUE 8 COLOR | 212 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 215 | 8 |  |  | V3-2  |
|  CUE 9 TYPE | 223 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 224 | 1 |  | RESERVED | V3-2  |
|  CUE 9 IN TIME | 225 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 9 OUT TIME | 229 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 233 | 1 |  |  | V3-2  |
|  CUE 9 COLOR | 234 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 237 | 8 |  |  | V3-2  |
|  CUE 10 TYPE | 245 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 246 | 1 |  | RESERVED | V3-2  |
|  CUE 10 IN TIME | 247 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 10 OUT TIME | 251 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 255 | 1 |  |  | V3-2  |
|  CUE 10 COLOR | 256 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 259 | 8 |  |  | V3-2  |

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
19

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

|  CUE 11 TYPE | 267 | 1 |  | Cue Type | V3-2  |
| --- | --- | --- | --- | --- | --- |
|  RESERVED | 268 | 1 |  | RESERVED | V3-2  |
|  CUE 11 IN TIME | 269 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 11 OUT TIME | 273 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 277 | 1 |  |  | V3-2  |
|  CUE 11 COLOR | 278 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 281 | 8 |  |  | V3-2  |
|  CUE 12 TYPE | 289 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 290 | 1 |  | RESERVED | V3-2  |
|  CUE 12 IN TIME | 291 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 12 OUT TIME | 295 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 299 | 1 |  |  | V3-2  |
|  CUE 12 COLOR | 300 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 303 | 8 |  |  | V3-2  |
|  CUE 13 TYPE | 311 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 312 | 1 |  | RESERVED | V3-2  |
|  CUE 13 IN TIME | 313 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 13 OUT TIME | 317 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 321 | 1 |  |  | V3-2  |
|  CUE 13 COLOR | 322 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 325 | 8 |  |  | V3-2  |
|  CUE 14 TYPE | 333 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 334 | 1 |  | RESERVED | V3-2  |
|  CUE 14 IN TIME | 335 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 14 OUT TIME | 339 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 343 | 1 |  |  | V3-2  |
|  CUE 14 COLOR | 344 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 347 | 8 |  |  | V3-2  |
|  CUE 15 TYPE | 355 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 356 | 1 |  | RESERVED | V3-2  |
|  CUE 15 IN TIME | 357 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 15 OUT TIME | 361 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 365 | 1 |  |  | V3-2  |
|  CUE 15 COLOR | 366 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 369 | 8 |  |  | V3-2  |
|  CUE 16 TYPE | 377 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 378 | 1 |  | RESERVED | V3-2  |
|  CUE 16 IN TIME | 379 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 16 OUT TIME | 383 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 387 | 1 |  |  | V3-2  |
|  CUE 16 COLOR | 388 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 391 | 8 |  |  | V3-2  |
|  CUE 17 TYPE | 399 | 1 |  | Cue Type | V3-2  |
|  RESERVED | 400 | 1 |  | RESERVED | V3-2  |
|  CUE 17 IN TIME | 401 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 17 OUT TIME | 405 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 409 | 1 |  |  | V3-2  |
|  CUE 17 COLOR | 410 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 413 | 8 |  |  | V3-2  |

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
20

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

|  CUE 18 TYPE | 421 | 1 |  | Cue Type | V3-2  |
| --- | --- | --- | --- | --- | --- |
|  RESERVED | 422 | 1 |  | RESERVED | V3-2  |
|  CUE 18 IN TIME | 423 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  CUE 18 OUT TIME | 427 | 4 | (LITTLE ENDIAN) | CUE IN Time | V3-2  |
|  RESERVED | 431 | 1 |  |  | V3-2  |
|  CUE 18 COLOR | 432 | 3 | BYTE1=RED, BYTE2=GREEN, BYTE 3=BLUE | CUE Color | V3-2  |
|  RESERVED | 435 | 8 |  |  | V3-2  |

# TCNet Data Packet - CUE Data - Details

Node ID: Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.

Protocol Version: Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00

Header: TCNet Protocol Header (Must be "TCN")

Message Type: Message type of packet. - Value=200

Node Name: GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH

SEQ: Sequence number of packet. (See Sequence number)

Node Type: Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater

Node Options: Node options: See Node Options

Timestamp: Time stamp in microseconds that is used to calculate network latency.

Data Type: Datatype of TCNet Data Packet. (Cue Data=12)

Layer ID: Layer number if layer sending data.
Example: LAYER1, 2=LAYER2, 3=LAYER3, 4=LAYER4, 5=LAYER A, 6=LAYER B, 7=MASTER OUT, 8=RESERVED

Loop IN: Time of Loop IN

Loop OUT: Time of Loop OUT

CUE Type: CUE Type

CUE IN Time: IN Time of CUE

CUE OUT Time: OUT Time of CUE

CUE COLOR: Cue Color (1^byte = RED (0-255, 2^byte = GREEN (0-255, 3^byte = BLUE (0-255)

Note:
This info can be requested from a node by sending a TCNet Request Data packet, with Datatype=12, Layer=The layer you request data from

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
21

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Data Packet - Small Wave Form Data

Functionality Contains Small Wave Form Data of layer
Type Unicast
Port UDP(Target-Node-Port)
Size 2442 (May change in future FLAMES)
Behavior Unicast upon request

|  PARAMETER | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 200 | Type 200: TCNet Data Packet | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name / Signature | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Data Type | 24 | 1 | 16 | Datatype 16 = Small Waveform | V3-2  |
|  Layer ID | 25 | 1 | 1-8 | Layer Number | V3-2  |
|  Data Size | 26 | 4 | Size=2400 (LITTLE ENDIAN) | Total Datasize | V3-2  |
|  Total Packet | 30 | 4 | 0-FF* | Total Packets used for data | V3-2  |
|  Packet No | 34 | 4 | (LITTLE ENDIAN) | Packet Number | V3-2  |
|  RESERVED | 38 | 4 |  | RESERVED | V3-2  |
|  Waveform Data | 42-2441 | 2400 | BLevel (Odd Bytes) / BColor (Even Bytes)-0-FF | Wave Form Data as Bar Levels/Bar Color Value | V3-2  |

* See details below:

# TCNet Data Packet – Small Wave Form Data - Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header TCNet Protocol Header (Must be "TCN")
Message Type Message type of packet. - Value=200
Node Name GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ Sequence number of packet. (See Sequence number)
Node Type Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options Node options: See Node Options
Timestamp Time stamp in microseconds that is used to calculate network latency.
Data Type Datatype of TCNet Data Packet. (Small Waveform=16)
Layer ID Layer number if layer sending data.
Example: LAYER1, 2=LAYER2, 3=LAYER3, 4=LAYER4, 5=LAYER A, 6=LAYER B, 7=MASTER OUT, 8=RESERVED
Total Packets Total number of packets for data (LITTLE ENDIAN)
Packet No Packet number of data
Data Size Total data size
Waveform Data Wave form data: Odd bytes are Bar Levels, Even bytes are Bar Colors (Total = 1200x2 = 2400 bytes)
BColor value is used to draw the intensity of a Color Bar. If you want to draw a blue waveform, the BColor can used as follow: RED=BColor Value, GREEN=BColor Value), BLUE=255.
This gives you the typical Pioneer DI Waveform look. If you want a green looking waveform, use: RED=BColor Value, GREEN=255, BLUE=Bcolor Value.

Note:
This info can be requested from a node by sending a TCNet Request Data packet, with Datatype=16, Layer=The layer you request data from

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TC™

# TCNet Data Packet – Big Wave Form Data

Functionality Contains Small Wave Form Data of layer
Type Unicast
Port UDP(Target-Node-Port)
Size Depending on track length
Behavior Unicast upon request

|  PARAMETER | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 200 | Type 200: TCNet Data Packet | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name / Signature | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Data Type | 24 | 1 | 32 | Datatype 32 = Big Waveform | V3-2  |
|  Layer ID | 25 | 1 | 1-8 | Layer Number | V3-2  |
|  Data Size | 26 | 4 | TOTAL DATA SIZE | Total Data size | V3-2  |
|  Total Packet | 30 | 4 | 0-FF* | Total Packets used for data | V3-2  |
|  Packet No | 34 | 4 | (LITTLE ENDIAN) | Packet Number | V3-2  |
|  Data Cluster Size | 38 | 4 | Standard: 4800 (LITTLE ENDIAN) | Data Cluster Size | V3-2  |
|  Waveform Data | 42 – Max 4842 | Max 4842 | BLevel (Odd Bytes) / BColor (Even Bytes)-0-FF | Wave Form Data as Bar Levels/Bar Colors | V3-2  |

* See details below:

# TCNet Data Packet – Big Wave Form Data – Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header TCNet Protocol Header (Must be "TCN")
Message Type Message type of packet. - Value=200
Node Name GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ Sequence number of packet. (See Sequence number)
Node Type Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options Node options: See Node Options
Timestamp Time stamp in microseconds that is used to calculate network latency.
Data Type Datatype of TCNet Data Packet. (Big Waveform=32)
Layer ID Layer number if layer sending data.
Example: LAYER1, 2=LAYER2, 3=LAYER3, 4=LAYER4, 5=LAYER A, 6=LAYER B, 7=MASTER OUT, 8=RESERVED
Total Packets Total amount of packets for data (LITTLE ENDIAN)
Packet No Packet number of data
Data Size Total data size. Is total of all data send, including in extra packets (LITTLE ENDIAN)
Data Cluster Size Cluster Size of data (Amount of bytes used per cluster to split up total data. - Standard value = 32000
Waveform Data Wave form data: Odd bytes are Bar Levels, Even bytes are Bar Colors
BColor value is used to draw the intensity of a Color Bar. If you want to draw a blue waveform, the BColor can used as follow: RED=BColor Value, GREEN=BColor Value), BLUE=255.
This gives you the typical Pioneer DJ Waveform look. If you want a green looking waveform, use: RED=BColor Value, GREEN=255, BLUE=Bcolor Value.

Note:
This info can be requested from a node by sending a TCNet Request Data packet, with Datatype=32, Layer=The layer you request data from

# TCNet Data Packet – Mixer Data

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TC

Functionality Updates Metrics Data for Layer

Type Unicast

Port UDP(Target-Node-Port)

Size 270

Behavior Unicast when cache changes.

|  PARAMETER | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 200 | Type 200: TCNet Data Packet | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name / Signature | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Data Type | 24 | 1 | 150 | Datatype 150 = Mixer Data | V3-5  |
|  Mixer ID | 25 | 1 | 0-FF* | Mixer ID | V3-5  |
|  Mixer Type | 26 | 1 | 0-FF* | Mixer Type | V3-5  |
|  RESERVED | 27 | 1 |  | RESERVED | V3-5  |
|  RESERVED | 28 | 1 |  | RESERVED | V3-5  |
|  Mixer Name | 29 | 16 | ASCII TEXT* | Name of Mixer | V3-5  |
|  RESERVED | 45 | 12 |  | RESERVED | V3-5  |
|  RESERVED | 57 | 2 |  | RESERVED FOR MIC 1-2 LEVEL | V3-5  |
|  Mic EQ Hi | 59 | 1 |  | Mic EQ HI | V3-5  |
|  Mic EQ Low | 60 | 1 |  | Mic EQ Low | V3-5  |
|  Master Audio Level | 61 | 1 | 0-255 | Master Audio Level | V3-5  |
|  Master Fader Level | 62 | 1 | 0-255 | Master Fader Level | V3-5  |
|  RESERVED | 63 | 4 |  | RESERVED | V3-5  |
|  Link Cue A | 67 | 1 | 0-1 | Link CUE A | V3-5  |
|  Link Cue B | 68 | 1 | 0-1 | Link CUE B | V3-5  |
|  Master Filter | 69 | 1 | 0-255 | Master Filter | V3-5  |
|  RESERVED | 70 | 1 |  | RESERVED | V3-5  |
|  Master CUE A | 71 | 1 | 0-1 | Master CUE A | V3-5  |
|  Master CUE B | 72 | 1 | 0-1 | Master CUE B | V3-5  |
|  RESERVED | 73 | 1 |  | RESERVED | V3-5  |
|  Master Isolator ON/FF | 74 | 1 | 0-1 | Master Isolator Switch | V3-5  |
|  Master Isolator Hi | 75 | 1 | 0-255 | Master Isolator Hi | V3-5  |
|  Master Isolator Mid | 76 | 1 | 0-255 | Master Isolator Mid | V3-5  |
|  Master Isolator Low | 77 | 1 | 0-255 | Master Isolator Low | V3-5  |
|  RESERVED | 78 | 1 |  | RESERVED | V3-5  |
|  Filter HPF | 79 | 1 | 0-255 | Filter HPF | V3-5  |
|  Filter LPF | 80 | 1 | 0-255 | Filter LPF | V3-5  |
|  Filter Resonance | 81 | 1 | 0-255 | Filter Resonance | V3-5  |
|  RESERVED | 82 | 2 |  | RESERVED | V3-5  |
|  Send FX Effect | 84 | 1 | 0-255 | Send FX Effect | V3-5  |

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
24

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

|   | Send FX Ext 1 | 85 | 1 | 0-1 | Send Return Ext 1 | V3-5  |
| --- | --- | --- | --- | --- | --- | --- |
|   |  Send FX Ext 2 | 86 | 1 | 0-1 | Send Return Ext 2 | V3-5  |
|   |  Send FX Master Mix | 87 | 1 | 0-1 | Send FX Master Mix | V3-5  |
|   |  Send FX Size Feedback | 88 | 1 | 0-255 | Send FX Size Feedback | V3-5  |
|   |  Send FX Time | 89 | 1 | 0-255 | Send FX Time | V3-5  |
|   |  Send FX HPF | 90 | 1 | 0-255 | Send FX HPF | V3-5  |
|   |  Send FX Level | 91 | 1 | 0-255 | Send FX Level | V3-5  |
|   |  Send Return 3 Source Select | 92 | 1 | 0-255 | Send Return 3 Source Select | V3-5  |
|   |  Send Return 3 Type | 93 | 1 | 0-255 | Send Return 3 Type | V3-5  |
|   |  Send Return 3 ON/OFF | 94 | 1 | 0-1 | Send Return 3 ON/OFF | V3-5  |
|   |  Send Return 3 Level | 95 | 1 | 0-1 | Send Return 3 Level | V3-5  |
|   |  RESERVED | 96 | 1 |  | RESERVED | V3-5  |
|   |  Channel Fader Curve | 97 | 1 | 0-2 | Channel Fader Curve | V3-5  |
|   |  Cross Fader Curve | 98 | 1 | 0-2 | Cross Fader Curve | V3-5  |
|   |  Cross Fader | 99 | 1 | 0-255 | Cross Fader | V3-5  |
|   |  BeatFX ON/OFF | 100 | 1 | 0-1 | BeatFX ON/OFF | V3-5  |
|   |  BeatFX Level/Depth | 101 | 1 | 0-255 | BeatFX Level/Depth | V3-5  |
|   |  BeatFX Channel Select | 102 | 1 | 0-255 | BeatFX Channel Select | V3-5  |
|   |  BeatFX Select | 103 | 1 | 0-255 | BeatFX Select | V3-5  |
|   |  BeatFX Freq Hi | 104 | 1 | 0-255 | BeatFX Frequency Hi | V3-5  |
|   |  BeatFX Freq Mid | 105 | 1 | 0-255 | BeatFX Frequency Mid | V3-5  |
|   |  BeatFX Freq Low | 106 | 1 | 0-255 | BeatFX Frequency Low | V3-5  |
|   |  Headphones Pre EQ | 107 | 1 | 0-255 | Headphones Pre EQ | V3-5  |
|   |  Headphones A Level | 108 | 1 | 0-255 | Headphones A Level | V3-5  |
|   |  Headphones A Mix | 109 | 1 | 0-255 | Headphones A Mix | V3-5  |
|   |  Headphones B Level | 110 | 1 | 0-255 | Headphones B Level | V3-5  |
|   |  Headphones B Mix | 111 | 1 | 0-255 | Headphones B Mix | V3-5  |
|   |  Booth Level | 112 | 1 | 0-255 | Booth Level | V3-5  |
|   |  Booth EQ Hi | 113 | 1 | 0-255 | Booth EQ Hi | V3-5  |
|   |  Booth EQ Low | 114 | 1 | 0-255 | Booth EQ Low | V3-5  |
|   |  RESERVED | 115 | 10 |  | RESERVED | V3-5  |
|   |  Channel 1 Source Select | 125 | 1 | 0-255 | Channel 1 Source Select | V3-5  |
|   |  Channel 1 Audio Level | 126 | 1 | 0-255 | Channel 1 Audio Level | V3-5  |
|   |  Channel 1 Fader Level | 127 | 1 | 0-255 | Channel 1 Fader Level | V3-5  |
|   |  Channel 1 Trim Level | 128 | 1 | 0-255 | Channel 1 Trim Level | V3-5  |
|   |  Channel 1 Comp Level | 129 | 1 | 0-255 | Channel 1 Compressor Level | V3-5  |
|   |  Channel 1 EQ Hi Level | 130 | 1 | 0-255 | Channel 1 EQ Hi Level | V3-5  |
|   |  Channel 1 EQ Hi Mid Level | 131 | 1 | 0-255 | Channel 1 EQ Hi Mid Level | V3-5  |
|   |  Channel 1 EQ Low Mid Level | 132 | 1 | 0-255 | Channel 1 Low Mid Level | V3-5  |
|   |  Channel 1 EQ Low Level | 133 | 1 | 0-255 | Channel 1 Low Level | V3-5  |
|   |  Channel 1 Filter/Color | 134 | 1 | 0-255 | Channel 1 Filter/Color | V3-5  |
|   |  Channel 1 Send | 135 | 1 | 0-255 | Channel 1 Send | V3-5  |
|   |  Channel 1 CUE A | 136 | 1 | 0-255 | Channel 1 CUE A | V3-5  |
|  Channel 1 CUE B | 137 | 1 | 0-255 | Channel 1 CUE B | V3-5  |   |
|  Channel 1 Crossfader Assign | 138 | 1 | 0-255 | Channel 1 Crossfader Assign | V3-5  |   |
|  RESERVED | 139 | 10 |  | RESERVED | V3-5  |   |
|  Channel 2 Source Select | 149 | 1 | 0-255 | Channel 2 Source Select | V3-5  |   |

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
25

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TC

|   | Channel 2 Audio Level | 150 | 1 | 0-255 | Chanel 2 Audio Level | V3-5  |
| --- | --- | --- | --- | --- | --- | --- |
|   |  Channel 2 Fader Level | 151 | 1 | 0-255 | Channel 2 Fader Level | V3-5  |
|   |  Channel 2 Trim Level | 152 | 1 | 0-255 | Channel 2 Trim Level | V3-5  |
|   |  Channel 2 Comp Level | 153 | 1 | 0-255 | Channel 2 Compressor Level | V3-5  |
|   |  Channel 2 EQ Hi Level | 154 | 1 | 0-255 | Channel 2 EQ Hi Level | V3-5  |
|   |  Channel 2 EQ Hi Mid Level | 155 | 1 | 0-255 | Channel 2 EQ Hi Mid Level | V3-5  |
|   |  Channel 2 EQ Low Mid Level | 156 | 1 | 0-255 | Channel 2 Low Mid Level | V3-5  |
|   |  Channel 2 EQ Low Level | 157 | 1 | 0-255 | Channel 2 Low Level | V3-5  |
|   |  Channel 2 Filter/Color | 158 | 1 | 0-255 | Channel 2 Filter/Color | V3-5  |
|   |  Channel 2 Send | 159 | 1 | 0-255 | Channel 2 Send | V3-5  |
|   |  Channel 2 CUE A | 160 | 1 | 0-255 | Channel 2 CUE A | V3-5  |
|   |  Channel 2 CUE B | 161 | 1 | 0-255 | Channel 2 CUE B | V3-5  |
|   |  Channel 2 Crossfader Assign | 162 | 1 | 0-255 | Channel 2 Crossfader Assign | V3-5  |
|   |  RESERVED | 163 | 10 | 0-255 | RESERVED | V3-5  |
|   |  Channel 3 Source Select | 173 | 1 | 0-255 | Channel 3 Source Select | V3-5  |
|   |  Channel 3 Audio Level | 174 | 1 |  | Chanel 3 Audio Level | V3-5  |
|   |  Channel 3 Fader Level | 175 | 1 | 0-255 | Channel 3 Fader Level | V3-5  |
|   |  Channel 3 Trim Level | 176 | 1 | 0-255 | Channel 3 Trim Level | V3-5  |
|   |  Channel 3 Comp Level | 177 | 1 | 0-255 | Channel 3 Compressor Level | V3-5  |
|   |  Channel 3 EQ Hi Level | 178 | 1 | 0-255 | Channel 3 EQ Hi Level | V3-5  |
|   |  Channel 3 EQ Hi Mid Level | 179 | 1 | 0-255 | Channel 3 EQ Hi Mid Level | V3-5  |
|   |  Channel 3 EQ Low Mid Level | 180 | 1 | 0-255 | Channel 3 Low Mid Level | V3-5  |
|   |  Channel 3 EQ Low Level | 181 | 1 | 0-255 | Channel 3 Low Level | V3-5  |
|   |  Channel 3 Filter/Color | 182 | 1 | 0-255 | Channel 3 Filter/Color | V3-5  |
|   |  Channel 3 Send | 183 | 1 | 0-255 | Channel 3 Send | V3-5  |
|   |  Channel 3 CUE A | 184 | 1 | 0-255 | Channel 3 CUE A | V3-5  |
|   |  Channel 3 CUE B | 185 | 1 | 0-255 | Channel 3 CUE B | V3-5  |
|   |  Channel 3 Crossfader Assign | 186 | 1 | 0-255 | Channel 3 Crossfader Assign | V3-5  |
|   |  RESERVED | 187 | 10 |  | RESERVED | V3-5  |
|   |  Channel 4 Source Select | 197 | 1 | 0-255 | Channel 4 Source Select | V3-5  |
|   |  Channel 4 Audio Level | 198 | 1 | 0-255 | Chanel 4 Audio Level | V3-5  |
|   |  Channel 4 Fader Level | 199 | 1 | 0-255 | Channel 4 Fader Level | V3-5  |
|   |  Channel 4 Trim Level | 200 | 1 | 0-255 | Channel 4 Trim Level | V3-5  |
|   |  Channel 4 Comp Level | 201 | 1 | 0-255 | Channel 4 Compressor Level | V3-5  |
|   |  Channel 4 EQ Hi Level | 202 | 1 | 0-255 | Channel 4 EQ Hi Level | V3-5  |
|   |  Channel 4 EQ Hi Mid Level | 203 | 1 | 0-255 | Channel 4 EQ Hi Mid Level | V3-5  |
|   |  Channel 4 EQ Low Mid Level | 204 | 1 | 0-255 | Channel 4 Low Mid Level | V3-5  |
|   |  Channel 4 EQ Low Level | 205 | 1 | 0-255 | Channel 4 Low Level | V3-5  |
|   |  Channel 4 Filter/Color | 206 | 1 | 0-255 | Channel 4 Filter/Color | V3-5  |
|   |  Channel 4 Send | 207 | 1 | 0-255 | Channel 4 Send | V3-5  |
|  Channel 4 CUE A | 208 | 1 | 0-255 | Channel 4 CUE A | V3-5  |   |
|  Channel 4 CUE B | 209 | 1 | 0-255 | Channel 4 CUE B | V3-5  |   |
|  Channel 4 Crossfader Assign | 210 | 1 | 0-255 | Channel 4 Crossfader Assign | V3-5  |   |
|  RESERVED | 211 | 10 |  | RESERVED | V3-5  |   |
|  Channel 5 Source Select | 221 | 1 | 0-255 | Channel 5 Source Select | V3-5  |   |
|  Channel 5 Audio Level | 222 | 1 | 0-255 | Chanel 5 Audio Level | V3-5  |   |
|  Channel 5 Fader Level | 223 | 1 | 0-255 | Channel 5 Fader Level | V3-5  |   |

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
26

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

|   | Channel 5 Trim Level | 224 | 1 | 0-255 | Channel 5 Trim Level | V3-5  |
| --- | --- | --- | --- | --- | --- | --- |
|   |  Channel 5 Comp Level | 225 | 1 | 0-255 | Channel 5 Compressor Level | V3-5  |
|   |  Channel 5 EQ Hi Level | 226 | 1 | 0-255 | Channel 5 EQ Hi Level | V3-5  |
|   |  Channel 5 EQ Hi Mid Level | 227 | 1 | 0-255 | Channel 5 EQ Hi Mid Level | V3-5  |
|   |  Channel 5 EQ Low Mid Level | 228 | 1 | 0-255 | Channel 5 Low Mid Level | V3-5  |
|   |  Channel 5 EQ Low Level | 229 | 1 | 0-255 | Channel 5 Low Level | V3-5  |
|   |  Channel 5 Filter/Color | 230 | 1 | 0-255 | Channel 5 Filter/Color | V3-5  |
|   |  Channel 5 Send | 231 | 1 | 0-255 | Channel 5 Send | V3-5  |
|   |  Channel 5 CUE A | 232 | 1 | 0-255 | Channel 5 CUE A | V3-5  |
|   |  Channel 5 CUE B | 233 | 1 | 0-255 | Channel 5 CUE B | V3-5  |
|   |  Channel 5 Crossfader Assign | 234 | 1 | 0-255 | Channel 5 Crossfader Assign | V3-5  |
|   |  RESERVED | 235 | 10 |  | RESERVED | V3-5  |
|   |  Channel 6 Source Select | 245 | 1 | 0-255 | Channel 6 Source Select | V3-5  |
|   |  Channel 6 Audio Level | 246 | 1 | 0-255 | Channel 6 Audio Level | V3-5  |
|   |  Channel 6 Fader Level | 247 | 1 | 0-255 | Channel 6 Fader Level | V3-5  |
|   |  Channel 6 Trim Level | 248 | 1 | 0-255 | Channel 4 Trim Level | V3-5  |
|   |  Channel 6 Comp Level | 249 | 1 | 0-255 | Channel 6 Compressor Level | V3-5  |
|   |  Channel 6 EQ Hi Level | 250 | 1 | 0-255 | Channel 6 EQ Hi Level | V3-5  |
|   |  Channel 6 EQ Hi Mid Level | 251 | 1 | 0-255 | Channel 6 EQ Hi Mid Level | V3-5  |
|   |  Channel 6 EQ Low Mid Level | 252 | 1 | 0-255 | Channel 6 Low Mid Level | V3-5  |
|   |  Channel 6 EQ Low Level | 253 | 1 | 0-255 | Channel 6 Low Level | V3-5  |
|   |  Channel 6 Filter/Color | 254 | 1 | 0-255 | Channel 6 Filter/Color | V3-5  |
|   |  Channel 6 Send | 255 | 1 | 0-255 | Channel 6 Send | V3-5  |
|   |  Channel 6 CUE A | 256 | 1 | 0-255 | Channel 6 CUE A | V3-5  |
|   |  Channel 6 CUE B | 257 | 1 | 0-255 | Channel 6 CUE B | V3-5  |
|   |  Channel 6 Crossfader Assign | 258 | 1 | 0-255 | Channel 6 Crossfader Assign | V3-5  |
|  RESERVED | 259 | 10 |  | RESERVED | V3-5  |   |

a See details next page

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
27

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TC™

# TCNet Data Packet - Mixer Data - Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.

Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00

Header TCNet Protocol Header (Must be "TCN")

Message Type Message type of packet. - Value=200

Node Name GW Code of software/machine/source that sends packet. (8 Characters)

Example: ABCDEFGH

SEQ Sequence number of packet. (See Sequence number)

Node Type Node Type

Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater

Node Options Node options: See Node Options

Timestamp Time stamp in microseconds that is used to calculate network latency.

Data Type Datatype of TCNet Data Packet. (Mixer Data = 150)

Mixer ID ID number of mixer sending data. (Standard = 0)

Mixer Type Mixer type (Standard = 0, Extended = 2)

Mixer Name Given name of Mixer

Mic EQ Hi / Low Value of Microphone EQ Hi / Low

Audio Level Value of Current Master Audio Level

Link CUE A Link A Cue button state

Link CUE B Link B Cue button state

Master Filter Value of Master Filter

Master Isolator ON/FF Value of Master Isolator (0=Off, 1=On)

Master Isolator H/M/L Value of Master Isolator EQ Parameter High/Mid/Low

Filter HPF/LPF Value of Highpass/Lowpass value of Master Filter

Filter Resonance Value of Resonance of Master Filter

Send FX Effect Current Effect selected for Send FX

Send FX Ext Current state of Send FX 1-2 (0=Off, 1=On)

Send FX Master Mix Value of Send FX Master Mix

Send FX Size Feedback Value of Send FX Feedback Size

Send FX Time Value of Send FX Time

Send FX HPF Value of Send FX HPF

Send FX Level Value of Send FX Level

Send Return 3 Source Setting of Send Return 3 Source switch (0=CH1, 1=CH2, 2=CH3, 3=CH4, 4=CH5, 5=CH6, 6=MIC, 7=MASTER 8=CRF A, 9=CRF B, 255=NONE)

Send Return 3 Type Setting Send Return 3 Source Type (0=USB-AUX, 1=USB-INSERT, 2=1/4" TS JACK-AUX, 3=1/4" TS JACK-INSERT, 255=NONE)

Send Return 3 ON/OFF Current state of Send Return 3 (0=Off, 1=On)

Send Return 3 Level Value of Send Return 3 Level

Channel Fader Curve Setting of Channel Fader Curve (1,2,3)

Cross Fader Curve Setting of Cross Fader Curve

Cross Fader Value of Cross Fader

BeatFX ON/OFF Setting of BeatFX (0=Off, 1=On)

BeatFX Level/Depth Value of BeatFX Level/Depth

BeatFX Channel Select Setting of BeatFX Channel (0=CH1, 1=CH2, 2=CH3, 3=CH4, 4=CH5, 5=CH6, 6=MIC, 7=MASTER, 8=CRF A, 9=CRF B, 255=NONE)

BeatFX Select Setting of BeatFX (0-13)

BeatFX Freq H/M/L Setting of BeatFX Freq High/Mid/Low (0=Off, 1=On)

Headphones Pre EQ Setting of Headphones Pre EQ (0=Off, 1=On)

Headphones Level Value of Headphones Level

Headphones Mix Value of Headphones Mix

Booth Level Value of Booth Level

Booth EQ Hi/Low Value Booth EQ Hi/Low

Channel Source Select Setting of Channel Source (0=USBA, 1=USBB, 2=DIGITAL, 3=LINE 4=PHONO, 5=INT, 6=RTN1, 7=RTN2, 8=RTN3, 9=RTN_ALL)

Channel Audio Level Value of Channel Level

Channel Fader Level Value of Channel Fader

Channel Trim Level Value of Channel Trim

Channel Comp Level Value of Channel Comp Level

Channel EQ Hi Level Value of Channel EQ Hi Level

Channel EQ Hi Mid Level Value of Channel EQ Hi Mid Level

Channel EQ Low Mid Level Value of Channel EQ Low Mid Level

Channel EQ Low Level Value of Channel EQ Low Level

Channel EQ Low Level Value of Channel EQ Low Level

Channel Filter/Color Value of Channel Filter/Color

Channel Send Value of Channel FX Send

Channel CUE A / B Setting of Channel CUE A / B (0=OFF, 1=ON)

Channel Crossfader Assign Setting Channel Crossfader Assign (0=THRU, 1=A, 2=B)

Note:
If you are not sure what mixer type to implement, use type 0.

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Data File Packet - Low Res Artwork File

Functionality Contains Low Res Artwork file (JPEG Format)
Type Unicast
Port UDP(Target-Node-Port)
Size Depending on file size
Behavior Unicast upon request

|  PARAMETER | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 204 | Type 204: TCNet File Data File Packet | V3-2  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name / Signature | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Data Type | 24 | 1 | 128 | Datatype 128 = Low Res Artwork File | V3-2  |
|  Layer ID | 25 | 1 | 1-8 | Layer Number | V3-2  |
|  Data Size | 26 | 4 | TOTAL DATA SIZE | Total Data size | V3-2  |
|  Total Packet | 30 | 4 | 0-FF* | Total Packets used for data | V3-2  |
|  Packet No | 34 | 4 | (LITTLE ENDIAN) | Packet Number | V3-2  |
|  Data Cluster Size | 38 | 4 | Standard: 4800 (LITTLE ENDIAN) | Data Cluster Size | V3-2  |
|  File Data | 42 – Max 4842 | Max 4842 | RAW FILE DATA | RAW FILE DATA | V3-2  |

* See details below:

# TCNet Data File Packet – Low Res Artwork File - Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header TCNet Protocol Header (Must be "TCN")
Message Type Message type of packet. - Value=204
Node Name GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ Sequence number of packet. (See Sequence number)
Node Type Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options Node options: See Node Options
Timestamp Timestamp in microseconds that is used to calculate network latency.
Data Type Datatype of TCNet Data Packet. (Low Res Artwork File=128)
Layer ID Layer number if layer sending data.
Example: LAYER1, 2=LAYER2, 3=LAYER3, 4=LAYER4, 5=LAYER A, 6=LAYER B, 7=MASTER OUT, 8=RESERVED
Total Packets Total number of packets for data (LITTLE ENDIAN)
Packet No Packet number of data
Data Size Total data size. Is total of all data send, including in extra packets (LITTLE ENDIAN)
Data Cluster Size Cluster Size of data (Amount of bytes used per cluster to split up total data. - Standard value = 32000
File Data Raw file data of JPEG file

Note:
This info can be requested from a node by sending a TCNet Request Data packet, with Datatype=12, Layer=The layer you request data from

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Application Specific Data Packet

Functionality: Application Specific Broadcasted Data
Type: Broadcast / Unicast
Port: UDP(60001) for Broadcast or Target-Node-Port for Unicast
Size: Data Size
Behavior: Broadcast or Unicast depending on application

|  PARAMETERS | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 30 | Type 30: TCNet Application Specific Data Packet | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name / Signature | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  Data Identifier 1 | 24 | 1 | 0-255 | Application Identifier Signature 1/2 (Defaults to 0) | V3-0  |
|  Data Identifier 2 | 25 | 1 | 0-255 | Application Identifier Signature 2/2 (Defaults to 0) | V3-0  |
|  Data Size | 26 | 4 | (LITTLE ENDIAN) | Data Size of all packets | V3-0  |
|  Total Packets | 30 | 4 | (LITTLE ENDIAN) | Total of all packets | V3-0  |
|  Packet No | 34 | 4 | (LITTLE ENDIAN) | Packet No | V3-0  |
|  Packet Signature | 38 | 4 | 178260640 (LITTLE ENDIAN) | Signature of Packet | V3-0  |
|  Data | 42 | Data Size |  | Data | V3-0  |

* See details below:

# TCNet Application Specific Data Packet - Details

Node ID: Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.
Protocol Version: Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00
Header: TCNet Protocol Header (Must be "TCN")
Message Type: Message type of packet. - Value=30
Node Name: GW Code of software/machine/source that sends packet. (8 Characters)
Example: ABCDEFGH
SEQ: Sequence number of packet. (See Sequence number)
Node Type: Node Type
Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater
Node Options: Node options: See Node Options
Timestamp: Time stamp in microseconds that is used to calculate network latency.
Application Identifier: 4 Byte Application Code. This code is used to identify application.
Data Size: Data Size in Little Endian
Total Packets: Total number of packets for data (LITTLE ENDIAN)
Packet No: Packet number of data
Packet Signature: Packet Signature (LITTLE ENDIAN)
Data: Data

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
30

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

# TCNet Time Packet

Functionality: Constant stream of timing data of layers
Type: Broadcast and Unicast to local host node's
Port: UDP(60001)
Size: 162 (May change in future FLAMES)
Behavior: Broadcast and Unicast every 1ms - 40ms or at time critical event.

|  PARAMETER | BYTE # | SIZE | VALUE | DESCRIPTION | FLAME  |
| --- | --- | --- | --- | --- | --- |
|  Management Header  |   |   |   |   |   |
|  Node ID | 0 | 2 | (LITTLE ENDIAN) | Node ID of sending device | V1-0  |
|  Protocol Version (Major) | 2 | 1 | 3 | Protocol Version (Major) of sending device | V1-0  |
|  Protocol Version (Minor) | 3 | 1 | 1 | Protocol Version (Minor) of sending device | V1-0  |
|  Header | 4 | 3 | TCN | TCNet Protocol Header | V3-1  |
|  Message Type | 7 | 1 | 254 | Type 254: TCNet Time Packet | V1-0  |
|  Node Name | 8 | 8 | ASCII TEXT* | Node Name / Signature | V3-1  |
|  SEQ | 16 | 1 | 0-255 | Sequence Number | V3-1  |
|  Node Type | 17 | 1 | 0-255 | Node Type | V3-1  |
|  Node Options | 18 | 2 | (LITTLE ENDIAN) | Node Options | V3-1  |
|  Timestamp in Microseconds | 20 | 4 | 0-999999 (LITTLE ENDIAN) | Timestamp in Microseconds | V3-1  |
|  Data  |   |   |   |   |   |
|  L1 Time | 24 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER 1 Current Time in Milliseconds | V3-0  |
|  L2 Time | 28 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER 2 Current Time in Milliseconds | V3-0  |
|  L3 Time | 32 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER 3 Current Time in Milliseconds | V3-0  |
|  L4 Time | 36 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER 4 Current Time in Milliseconds | V3-0  |
|  LA Time | 40 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER A Current Time in Milliseconds | V3-0  |
|  LB Time | 44 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER B Current Time in Milliseconds | V3-0  |
|  LM Time | 48 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER M Current Time in Milliseconds | V3-0  |
|  LC Time | 48 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER C Current Time in Milliseconds | V3-0  |
|  L1 Total Time | 56 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER 1 Total Time in Milliseconds | V3-0  |
|  L2 Total Time | 60 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER 2 Total Time in Milliseconds | V3-0  |
|  L3 Total Time | 64 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER 3 Total Time in Milliseconds | V3-0  |
|  L4 Total Time | 68 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER 4 Total Time in Milliseconds | V3-0  |
|  LA Total Time | 72 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER A Total Time in Milliseconds | V3-0  |
|  LB Total Time | 76 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER B Total Time in Milliseconds | V3-0  |
|  LM Total Time | 80 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER M Total Time in Milliseconds | V3-0  |
|  LC Total Time | 84 | 4 | 0-0x55D4A80 (LITTLE ENDIAN) | LAYER C Total Time in Milliseconds | V3-  |
|  L1 Beat Marker | 88 | 1 | 0-4 | Layer 1 Beatmarker | V3-0  |
|  L2 Beat Marker | 89 | 1 | 0-4 | Layer 2 Beatmarker | V3-0  |
|  L3 Beat Marker | 90 | 1 | 0-4 | Layer 3 Beatmarker | V3-0  |
|  L4 Beat Marker | 91 | 1 | 0-4 | Layer 4 Beatmarker | V3-0  |
|  LA Beat Marker | 92 | 1 | 0-4 | Layer A Beatmarker | V3-0  |
|  LB Beat Marker | 93 | 1 | 0-4 | Layer B Beatmarker | V3-0  |
|  LM Beat Marker | 94 | 1 | 0-4 | Layer M Beatmarker | V3-0  |
|  LC Beat Marker | 94 | 1 | 0-4 | Layer C Beatmarker | V3-0  |
|  L1 Layer State | 96 | 1 | 0-FF | Layer 1 Layer State | V3-0  |
|  L2 Layer State | 97 | 1 | 0-FF | Layer 2 Layer State | V3-0  |
|  L3 Layer State | 98 | 1 | 0-FF | Layer 3 Layer State | V3-0  |
|  L4 Layer State | 99 | 1 | 0-FF | Layer 4 Layer State | V3-0  |
|  LA Layer State | 100 | 1 | 0-FF | Layer A State | V3-0  |
|  LB Layer State | 101 | 1 | 0-FF | Layer B State | V3-0  |
|  LM Layer State | 102 | 1 | 0-FF | Layer M State | V3-0  |

* Resumes next page

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
31

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Time Packet

* Resumed from previous page

Data (Resume)

|  LC Layer State | 103 | 1 | 0-FF | Layer C State | V3-0  |
| --- | --- | --- | --- | --- | --- |
|  RESERVED | 104 | 1 |  |  | V2-0  |
|  SMPTE Mode | 105 | 1 | = 24 or 25 or 29 or 30 | General SMPTE Mode | V2-0  |
|  L1 SMPTE Mode | 106 | 1 | = 24 or 25 or 29 or 30 | Layer SMPTE Mode | V2-0  |
|  L1 Time Code State | 107 | 1 | 0-2 | Time Code State | V2-0  |
|  L1 Time Code Hours | 108 | 1 | 0-17 | Time Code Hours | V2-0  |
|  L1 Time Code Minutes | 109 | 1 | 0-3B | Time Code Minutes | V2-0  |
|  L1 Time Code Seconds | 110 | 1 | 0-3B | Time Code Seconds | V2-0  |
|  L1 Time Code Frames | 111 | 1 | 0-1D | Time Code Frames | V2-0  |
|  L2 SMPTE Mode | 112 | 1 | = 24 or 25 or 29 or 30 | Layer SMPTE Mode | V2-0  |
|  L2 Time Code State | 113 | 1 | 0-2 | Time Code State | V2-0  |
|  L2 Time Code Hours | 114 | 1 | 0-17 | Time Code Hours | V2-0  |
|  L2 Time Code Minutes | 115 | 1 | 0-3B | Time Code Minutes | V2-0  |
|  L2 Time Code Seconds | 116 | 1 | 0-3B | Time Code Seconds | V2-0  |
|  L2 Time Code Frames | 117 | 1 | 0-1D | Time Code Frames | V2-0  |
|  L3 SMPTE Mode | 118 | 1 | = 24 or 25 or 29 or 30 | Layer SMPTE Mode | V2-0  |
|  L3 Time Code State | 119 | 1 | 0-2 | Time Code State | V2-0  |
|  L3 Time Code Hours | 120 | 1 | 0-17 | Time Code Hours | V2-0  |
|  L3 Time Code Minutes | 121 | 1 | 0-3B | Time Code Minutes | V2-0  |
|  L3 Time Code Seconds | 122 | 1 | 0-3B | Time Code Seconds | V2-0  |
|  L3 Time Code Frames | 123 | 1 | 0-1D | Time Code Frames | V2-0  |
|  L4 SMPTE Mode | 124 | 1 | = 24 or 25 or 29 or 30 | Layer SMPTE Mode | V2-0  |
|  L4 Time Code State | 125 | 1 | 0-2 | Time Code State | V2-0  |
|  L4 Time Code Hours | 126 | 1 | 0-17 | Time Code Hours | V2-0  |
|  L4 Time Code Minutes | 127 | 1 | 0-3B | Time Code Minutes | V2-0  |
|  L4 Time Code Seconds | 128 | 1 | 0-3B | Time Code Seconds | V2-0  |
|  L4 Time Code Frames | 129 | 1 | 0-1D | Time Code Frames | V2-0  |

* Resumes next page

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
32

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Time Packet

* Resumed from previous page

Data (Resume)

|  LA SMPTE Mode | 130 | 1 | = 24 or 25 or 29 or 30 | Layer SMPTE Mode | V2-0  |
| --- | --- | --- | --- | --- | --- |
|  LA Time Code State | 131 | 1 | 0-2 | Time Code State | V2-0  |
|  LA Time Code Hours | 132 | 1 | 0-17 | Time Code Hours | V2-0  |
|  LA Time Code Minutes | 133 | 1 | 0-3B | Time Code Minutes | V2-0  |
|  LA Time Code Seconds | 134 | 1 | 0-3B | Time Code Seconds | V2-0  |
|  LA Time Code Frames | 135 | 1 | 0-1D | Time Code Frames | V2-0  |
|  LB SMPTE Mode | 136 | 1 | = 24 or 25 or 29 or 30 | Layer SMPTE Mode | V2-0  |
|  LB Time Code State | 137 | 1 | 0-2 | Time Code State | V2-0  |
|  LB Time Code Hours | 138 | 1 | 0-17 | Time Code Hours | V2-0  |
|  LB Time Code Minutes | 139 | 1 | 0-3B | Time Code Minutes | V2-0  |
|  LB Time Code Seconds | 140 | 1 | 0-3B | Time Code Seconds | V2-0  |
|  LB Time Code Frames | 141 | 1 | 0-1D | Time Code Frames | V2-0  |
|  LM SMPTE Mode | 142 | 1 | = 24 or 25 or 29 or 30 | Layer SMPTE Mode | V2-0  |
|  LM Time Code State | 143 | 1 | 0-2 | Time Code State | V2-0  |
|  LM Time Code Hours | 144 | 1 | 0-17 | Time Code Hours | V2-0  |
|  LM Time Code Minutes | 145 | 1 | 0-3B | Time Code Minutes | V2-0  |
|  LM Time Code Seconds | 146 | 1 | 0-3B | Time Code Seconds | V2-0  |
|  LM Time Code Frames | 147 | 1 | 0-1D | Time Code Frames | V2-0  |
|  LC SMPTE Mode | 148 | 1 | = 24 or 25 or 29 or 30 | Layer SMPTE Mode | V2-0  |
|  LC Time Code State | 149 | 1 | 0-2 | Time Code State | V2-0  |
|  LC Time Code Hours | 150 | 1 | 0-17 | Time Code Hours | V2-0  |
|  LC Time Code Minutes | 151 | 1 | 0-3B | Time Code Minutes | V2-0  |
|  LC Time Code Seconds | 152 | 1 | 0-3B | Time Code Seconds | V2-0  |
|  LC Time Code Frames | 153 | 1 | 0-1D | Time Code Frames | V2-0  |

* Resumes next page

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
33

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCnet

# TCNet Time Packet

* Resumed from previous page

Data (Resume)

|  L1 Layer OnAir | 154 | 1 | 0-255 | Layer OnAir State | V3-3-3  |
| --- | --- | --- | --- | --- | --- |
|  L2 Layer OnAir | 155 | 1 | 0-255 | Layer OnAir State | V3-3-3  |
|  L3 Layer OnAir | 156 | 1 | 0-255 | Layer OnAir State | V3-3-3  |
|  L4 Layer OnAir | 157 | 1 | 0-255 | Layer OnAir State | V3-3-3  |
|  LA Layer OnAir | 158 | 1 | 0-255 | Layer OnAir State | V3-3-3  |
|  LB Layer OnAir | 159 | 1 | 0-255 | Layer OnAir State | V3-3-3  |
|  LM Layer OnAir | 160 | 1 | 0-255 | Layer OnAir State | V3-3-3  |
|  LC Layer OnAir | 161 | 1 | 0-255 | Layer OnAir State | V3-3-3  |

* Resumes next page

# TCNet Time Packet – Details

Node ID Unique Node ID. When multiple applications/services are running on same IP, this number must be unique.

Protocol Version Protocol version of source that sends the packet. - Example: 1.00 would be: 01 00

Header TCNet Protocol Header (Must be "TCN")

Message Type Message type of packet. - Value=254

Node Name GW Code of software/machine/source that sends packet. (8 Characters)

Example: ABCDEFGH

SEQ Sequence number of packet. (See Sequence number)

Node Type Node Type

Example: 1=Auto, 2=Master, 4=Slave, 8=Repeater

Node Options Node options: See Node Options

Timestamp Time stamp in microseconds that is used to calculate network latency.

LX Time Layer X Time in MS

LX Total Time Layer X Total time in MS

LX Beatmarker Layer X Beatmarker position (0=unknown, 1-4=Beatmarker pos)

LX Layer State Layer X Layer State (Example: 0=IDLE, 3=PLAYING, 4=LOOPING, 5=PAUSED, 6=STOPPED, 7=CUE BUTTON DOWN, 8=PLATTER DOWN, 9=FFWD, 10=FFRV, 11=HOLD)

SMPTE Mode SMPTE Mode set on node

Values: 24=24FPS, 25=25FPS, 29=29.7FPS, 30=30FPS

LX Layer OnAir Layer X Layer OnAir State, Fader Position (0-255) (Example: 0=Not on Air, &gt;=1 =On Air)

LX SMPTE Mode SMPTE Mode. If value =0, Use SMPTE general. SMPTE mode defined in byte 105

LX TC State Status of timecode embedded in packet (Values: 0=Stopped, 1=Running, 2=Force Re sync)

LX TC Hours Hours value of the timecode (Values: 0–23)

LX TC Minutes Minutes value of the timecode (Values: 0–59)

LX TC Seconds Seconds value of the timecode (Values: 0–23)

LX TC Frames Frames value of the timecode (Values: 0 – Depending on frame rate)

# TCNet Time Packet – Usage

In order to correctly implement the Time Packet usage, the following steps are needed.

Step 1:

Create a Time Packet

Step 2:

Broadcast Time Packet to port 60001

Step 3:

Unicast Time Packet to each discovered LOCAL node, targeting that node's port. This ensures that when a node doesn't receive broadcast messages, it still can receive these packets.

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TC™

# Contribute

Contribution can be done by submitting your changes/idea's to info@eiglive.com.

# Special thanks to all contributors!

Alex Vincente, Alex Wilson, Arjan van Vught, Bart van der Ploeg, Brady Villadsen, Christiano Alvarez, Christian Jackson, David Moor, Fraser Stockley, Freek met een B, Ian Alvarez, James Dutton, Jason Buckley, Kevin Longwell, Koen de Puysseleir, Lars Schlichting, Laura Moor, Marco Hinic, Matt Matthews, Michael Hicks, Oliver Suckling, Oliver Waits, Simon Evans, Tim Walther, Uwe Schroeder, Yasuhiko Akita

Registered Application Codes

If you require an Application code, please contact dev@eiglive.com

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
35

TCNet

LINK SPECIFICATION V3.5.1B – 02/03/2022

TC

|  0000 | Reserved for Public |   |
| --- | --- | --- |
|  0AA0 | Pioneer DI | http://www.pioneerdj.com  |
|  0AAA | TC Supply / ShowKontrol | http://www.showkontrol.com  |
|  0AAB | TC Supply Pyrotechnic Systems | http://www.tc-supply.com  |
|  0AAC | TC Supply Ride Control Systems | http://www.tc-supply.com  |
|  0AB0 | Avolites Lighting | http://www.avolites.com  |
|  0AB1 | MA Lighting | http://www.malighting.com  |
|  0AB3 | Chamsys Lighting | http://www.chamsys.co.uk  |
|  0AB4 | Obsidian Control | http://www.obsidiancontrol.com  |
|  0ABA | Arkaos Software | http://www.arkaos.net  |
|  0ABB | BLCKBOOK / Time Code Sync | http://www.timecodesync.com  |
|  0ABC | Resolume Software | http://www.resolume.com  |
|  0ABD | Green Hippo | http://www.green-hippo.com  |
|  0ABE | RD/ShowCockpit | http://www.showcockpit.com  |
|  0ABF | Disguise | http://disguise.one  |
|  0ACA | OrangePI | http://orangepi.dmx.org  |
|  0ACB | RedPill VR | http://www.redpillvr.com  |
|  FFFF | Reserved for Public |   |

# TCNet Change Log

|  02/03/22 | Document Correction 3.5.1 (!) UTF32 supposed to be UTF16 in Metadata Packet.  |
| --- | --- |
|  09/07/21 | Revision 3.5.1 Added CUE Data for Hot/Memory Cue's  |
|  03/13/21 | Revision 3.5.0  |
|  06/22/20 | Revision 3.4.2 Added Extended Data to Mixer Data Packet (Type 150)  |
|  12/03/19 | Revision 3.4.1 Added Mixer Data Packet (Type 150)  |
|  12/03/19 | Revision 3.4.1 Changed Revision number to 3.4.1 to ensure correct backward compatibility.  |
|  11/11/19 | Revision 3.3.3 Added Fader values to On Air bytes in Time Packets  |
|  11/11/19 | Revision 3.3.3 Added Unicast to Opt-IN, Opt-OUT and Time Packets  |
|  11/11/19 | Revision 3.3.3 Added Explanation of Bcolor data in Small/Big waveform packets  |
|  10/13/19 | Revision 3.3.2 Added Layer Name in Layer Status packets  |
|  05/28/19 | Revision 3.3.1 Added back SMPTE values in timing packets, Updated list of Registered Application Codes  |
|  04/29/19 | Revision 3.3.0 Added Status Packets, Updated list of Registered Application Codes  |
|  03/28/19 | Revision 3.2.8 Removed SMPTE format from timing packets  |
|  02/18/19 | Revision 3.2.8 Added Node Options  |
|  02/15/19 | Revision 3.2.7 Clean UP  |
|  01/10/19 | Revision 3.2.6 Clean Up  |
|  12/24/18 | Revision 3.2.5 Added Artwork File, Cue Data, Removed Timecode format from timing packet (deprecated)  |
|  12/20/18 | Revision 3.2.4 Clean Up  |
|  12/17/18 | Revision 3.2.4 Clean Up  |
|  11/27/18 | Revision 3.2.2 Clean Up  |
|  11/19/18 | Revision 3.2.1 Added Beat Grid Info Packet  |
|  11/10/18 | Revision 3.2.0 Replaced Small Wave Form Packets to 3.2.0  |
|  10/31/18 | Revision 3.1.6 (PRE FINAL)  |
|  10/26/18 | Revision 3.1.5 (PRE FINAL)  |
|  09/28/18 | Revision 3.1.3  |
|  02/02/18 | Revision 3.0.0  |
|  12/21/17 | Revision 2.1.0  |
|  10/01/17 | Added Flame V2-0  |
|  05/10/16 | Added Flame V1.0 (REV D)  |
|  01/17/16 | Document Creation  |

©2016 – 2022 Event Imagineering Group – Development

LINK SPECIFICATION V3.5.1B – 02/03/2022
36

