use std::net::{Ipv4Addr, Ipv6Addr};
use radius::value_pair;
use try_from::TryFrom;
use radius::*;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::str::Utf8Error;
use num::FromPrimitive;

quick_error! {
    #[derive(Debug,Clone,Copy)]
    pub enum ConversionError {
        ValuePair {
            description("Error converting value_pair")
            display("Error converting value_pair")
            from(Utf8Error)
        }
    }
}

#[derive(Debug)]
pub enum Attribute {
    UserName(String),
    UserPassword(String),
    ChapPassword(String),
    NasIpAddress(Ipv4Addr),
    NasPort(u32),
    ServiceType(ServiceType),
    FramedProtocol(Protocol),
    FramedIpAddress(Ipv4Addr),
    FramedRouting(RoutingValue),
    FilterId(u32),
    FramedMtu(u32),
    FramedCompression(CompressionType),
    LoginIpHost(Ipv4Addr),
    LoginService(LoginService),
    LoginPort(u32),
    OldPassword(String), //deprecated
    ReplyMessage(String),
    LoginCallbackNumber(String),
    Expiration(u32),     //deprecated
    FramedRoute(String),
    FramedIpxNetwork(u32),
    State(String),
    Class(String),
    VendorSpecific(String),
    SessionTimeout(u32),
    IdleTimeout(u32),
    TerminationAction(TerminationAction),
    CalledStationId(String),
    CallingStationId(String),
    NasIdentifier(String),
    ProxyState(String),
    LoginLatService(String),
    LoginLatNode(String),
    LoginLatGroup(String),
    FramedAppletalkLink(String),
    FramedAppletalkNetwork(String),
    FramedAppletalkZone(String),
    EventTimestamp(u32),
    ChapChallenge(String),
    NasPortType(NasPortType),
    PortLimit(u32),
    LoginLatPort(String),
    ConnectInfo(String),
    MessageAuthenticator(String),
    NasIpv6Address(Ipv6Addr),
    FramedInterfaceId(String),
    FramedIpv6Prefix(String),
    LoginIpv6Host(String),
    FramedIpv6Route(String),
    FramedIpv6Pool(String),
    FramedIpv6Address(Ipv6Addr),
    DnsServerIpv6Address(Ipv6Addr),
    RouteIpv6Information(String),
    AcctStatusType(StatusType),
    AcctDelayTime(u32),
    AcctInputOctets(u32),
    AcctOutputOctets(u32),
    AcctSessionId(String),
    AcctAuthentic(Authentic),
    AcctSessionTime(u32),
    AcctInputPackets(u32),
    AcctOutputPackets(u32),
    AcctTerminateCause(AccountTerminationCause),
    AcctMultiSessionId(u32),
    AcctLinkCount(u32),
    AcctInputGigawords(u32),
    AcctOutputGigawords(u32),

    //Experimental SIP-specific attributes
    DigestResponse(String),
    DigestAttributes(String),
    DigestRealm(String),
    DigestNonce(String),
    DigestMethod(String),
    DigestUri(String),
    DigestQop(String),
    DigestAlgorithm(String),
    DigestBodyDigest(String),
    DigestCnonce(String),
    DigestNonceCount(String),
    DigestUserName(String),

    //Merit Experimental Extensions
    UserId(String),
    UserRealm(String)
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum AttributeType {
    String     = 0,
    Intger     = 1,
    IpAddress  = 2,
    Date       = 3,
    Ipv6Addr   = 4,
    Ipv6Prefix = 5
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum ServiceType {
    Login             = 1,
    Framed            = 2,
    CallbackLogin     = 3,
    CallbackFramed    = 4,
    Outbound          = 5,
    Administrative    = 6,
    NasPrompt         = 7,
    AuthenticateOnly  = 8,
    CallbackNasPrompt = 9
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum Protocol {
    Ppp      = 1,
    Slip     = 2,
    Ara      = 3,
    Gandalf  = 4,
    Xylogics = 5,
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum RoutingValue {
    None            = 0,
    Broadcast       = 1,
    Listen          = 2,
    BroadcastListen = 3
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum CompressionType {
    VanJacobsenTcpIp     = 1,
    IpxHeaderCompression = 2
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum LoginService {
    Telnet     = 0,
    Rlogin     = 1,
    TcpClear   = 2,
    Portmaster = 3,
    Lat        = 4,
    X25Pad     = 5,
    X25T3pos   = 6
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum TerminationAction {
    Default       = 0,
    RadiusRequest = 1,
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum ProhibitProtocol {
    Dumb     = 0,
    Ppp      = 1,
    Slip     = 2,
    AuthOnly = 3,
    All      = 255
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum StatusType {
    Start         = 1,
    Stop          = 2,
    Alive         = 3,
    ModemStart    = 4,
    ModemStop     = 5,
    StatusCancel  = 6,
    AccountingOn  = 7,
    AccountingOff = 8,
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum AccountTerminationCause {
    UserRequest        = 1,
    LostCarrier        = 2,
    LostService        = 3,
    AcctIdleTimeout    = 4,
    AcctSessionTimeout = 5,
    AdminReset         = 6,
    AdminReboot        = 7,
    PortError          = 8,
    NasError           = 9,
    NasRequest         = 10,
    NasReboot          = 11,
    PortUnneeded       = 12,
    PortPreempted      = 13,
    PortSuspended      = 14,
    ServiceUnavailible = 15,
    Callback           = 16,
    UserError          = 17,
    HostRequest        = 18
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum NasPortType {
    Async        = 0,
    Sync         = 1,
    IsdnSync     = 2,
    IsdnSyncV120 = 3,
    IsdnSyncV110 = 4,
    Virtual      = 5
}

#[derive(Debug,ToPrimitive,FromPrimitive,Copy,Clone)]
pub enum Authentic {
    Radius = 1,
    Local  = 2,
    Remote = 3,
}

impl TryFrom<value_pair> for Attribute {
    type Err = ConversionError;
    fn try_from(attr: value_pair) -> Result<Self,ConversionError> {
        let attribute_type = AttributeType::from_i32(attr.type_).ok_or(ConversionError::ValuePair)?;
        let string_value = if attr.type_ as u32 != PW_TYPE_INTEGER
                           && attr.type_ as u32 != PW_TYPE_DATE {
            unsafe {CStr::from_ptr(&attr.strvalue as *const c_char).to_str()?}
        } else {
            ""
        };
        Ok(match (attr.attribute as u32, attribute_type) {
            (PW_USER_NAME, AttributeType::String) => Attribute::UserName(string_value.to_owned()),
            (PW_USER_PASSWORD, AttributeType::String) => Attribute::UserPassword(string_value.to_owned()),
            (PW_CHAP_PASSWORD, AttributeType::String) => Attribute::ChapPassword(string_value.to_owned()),
            (PW_NAS_IP_ADDRESS, AttributeType::IpAddress) => Attribute::NasIpAddress(attr.lvalue.into()),
            (PW_NAS_PORT, AttributeType::IpAddress) => Attribute::NasPort(attr.lvalue),

            _ => return Err(ConversionError::ValuePair)
        })
    }
}
