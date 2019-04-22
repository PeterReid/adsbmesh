### Subscribe
The sender expresses intent to partner with the recipient. It generates a partnering ID which should be unique to both the partners.

The subscribe message includes, unencrypted and in the clear, a private key unique to this partnership.

TODO: Should the partnership key be encrypted using the receiver's private key? There is not an obvious choice for a PK algorithm that would not need to be replaced eventually, and algorithm agility and key rotation would be huge complications that might lead to vulnerabilities of their own. If the threat model excludes a "global malevolent eavesdropper" character, then this is not important. There is no inherent trust in a partner, just an expectation that past reliability predicts future reliability.



Format: 0x01 [Partnering ID: U32LE] [Partnership Private Key: u8x32] [Contact Port: U16LE] [Contact IP/Hostname]

### Subscribe Decline
The sender replies to a `Subscribe` message to express that it will not begin a partnership with the recipient. The sender can suggest a duration after which to retry and send another `Subscribe` request, in seconds. 

If the sender is declining the request because of an ID collision with an existing Partnering ID, it may use a RetryInterval of 0 so that the connecting can be tried again with another ID.

If the sender wants to permanently decline the subscription request, the maximum RetryInterval of 0xFFFFFFFF seconds, or about 136 years, should suffice.

Although the RetryInterval is only a suggestion to the would-be partner, sending a `Subscribe` that ignores the RetryInterval might discredit the sender and make a Subscribe Accept response less likely.

Format: 0x02 [Partnering ID: U32LE] [RetryInterval: U32LE]

### Subscribe Accept
A sender replies to a `Subscribe` messages with a `Subscribe Accept` messages to express that it will begin a partnership with the recipient. The `Subscribe Accept`'s Partnering ID is a copy of that from the triggering `Subscribe` message.

The `Confirmation Nonce` is randomly generated. This field is to prevent a (possibly malicious) `Subscribe` message with incorrect contact details from causing subscription-related traffic to be sent to an unsuspecting host.

Format 0x03 [Partnering ID: U32LE] [Confirmation Nonce: U32LE]

### Subscribe Finalize
A sender replies to a `Subscribe Accept` messages with a `Subscribe Finalize` message in order to finish setting up the partnership. The partnership ID and confirmation nonce are copies of that from the `Subscribe Accept` message.

Format 0x04 [Partnering ID: U32LE] [Confirmation Nonce: U32LE]

### Data
When an ADSB packet is received, a node sends out a `Data` messages to all active partners.

The sequence number is designed to make a compromised partnering private key more obvious, so that the recipient can send an `Unsubscribe` message. A node should increment the sequence number between packets whenever possible. A sequence counter does not need to be permanently persisted and receivers should tolerate occasional jumps in the sequence number. A receiver should also tolerate some re-ordering of `Data` by the network. Warning signs of private key compromise could include alternating between wildly different sequence numbers or frequently receiving repeated sequence numbers. *Node policy point: detect compromised private partnering key based the the most recent N (time, sequence number)s received from and signed by a partner.*

An empty [Packet] may be sent as a "keep alive" to show a partner that this node is active even if it has no data. 

Format: 0x05 [Partnering ID: U32LE] [Signature] [Sequence number: U32LE] [Packet]

### Profile Request
Format: 0x08 [Request Token: U32LE] [Start Index: U32LE] [0 Padding]

There should be as many 0-padding bytes as the sender hopes to receive from the profile string. This is to prevent a denial of service amplification, where a sender would forge the sender field on `Profile Request` packets to have larger `Profile Response` packets sent to the denial of service victim.

### Profile Response
Format: 0x09 [Request Token: U32LE] [Start Index: U32LE] [Profile Substring]

### Partner List Request

A node maintains a string that represents its list of partners, with each partner formatted as 

[Port Hex Digit 1] [Port Hex Digit 2] [Port Hex Digit 3] [Port Hex Digit 4] [IP Address or hostname] 0x00

and the string simply being the concatenation of those. The IP address or hostname cannot include a 0x00 byte. The port is formatted with the most significant digit first and uses upper case hexadecimal characters. (The hexadecimal encoding is so that a 0 is unambiguously a separating character).

A node can request a substring of the partner list string with the `Partner List Request` message.

There should be as many 0-padding bytes as the sender hopes to receive from the partner list string. This is to prevent a denial of service amplification, where a sender would forge the sender field on `Partner List Request` packets to have larger `Partner List Response` packets sent to the denial of service victim.


Format: 0x0A [Request Token: U32LE] [Start Index: U32LE] [0 Padding]

### Partner List Response

The receiver of this message should note that the string it is receiving a substring of may not be the same string as it received a substring of earlier. Simply concatenating responses to a batch of requests will not necessarily create a coherent whole. To ensure that each element is intact, a receiver might only process elements that do not cross response boundaries, and might choose boundaries to avoid splitting elements.

Format: 0x0A [Request Token: U32LE] [Start Index: U32LE] [Profile List Substring]

# A note about unsubscribing

For simplicity and robustness, there is no explicit `Unsubscribe` message. A working partnership involves active participation from both sides, and if one side stops participating then the other side will as well after a time. In particular, a node can test for a "not participating" partner by noting that it is online and responsive to `Profile Request` messages but is not sending out any `Data` messages, even empty "keep alive" ones. The exact criteria for ending a subscription is a policy choice.


