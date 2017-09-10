use std::net::{Ipv4Addr, Ipv6Addr};
use radius::value_pair;
use try_from::TryFrom;
use radius::*;
use std::str::Utf8Error;
use num::FromPrimitive;
use num::ToPrimitive;

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
    FramedIpNetmask(Ipv4Addr),
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
    Integer     = 1,
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

impl Attribute {
    fn attribute_type(&self) -> AttributeType {
        match *self {
            Attribute::UserPassword(..) |
            Attribute::UserName(..) => AttributeType::String,
            _ => unimplemented!(),
        }
    }

    fn into(self, handle : rc_handle) -> Box<value_pair> {
        match self.attribute_type() {
            AttributeType::String => {
                unsafe {
                    Box::from_raw(rc_avpair_add(handle,self.to_u32().unwrap(),))
                }
            }
        }
    }

    fn to_string_value(Self) {
        match Self {
            Attribute::UserName(s)|
            Attribute::UserPassword(s)|
            Attribute::ChapPassword(s)|
            Attribute::OldPassword(s)|
            Attribute::ReplyMessage(s)|
            Attribute::LoginCallbackNumber(s)|
            Attribute::FramedRoute(s)|
            Attribute::State(s)|
            Attribute::Class(s)|
            Attribute::VendorSpecific(s)|
            Attribute::CalledStationId(s)|
            Attribute::CallingStationId(s)|
            Attribute::NasIdentifier(s)|
            Attribute::ProxyState(s)|
            Attribute::LoginLatService(s)|
            Attribute::LoginLatNode(s)|
            Attribute::LoginLatGroup(s)|
            Attribute::FramedAppletalkLink(s)|
            Attribute::FramedAppletalkNetwork(s)|
            Attribute::FramedAppletalkZone(s)|
            Attribute::ChapChallenge(s)|
            Attribute::LoginLatPort(s)|
            Attribute::ConnectInfo(s)|
            Attribute::MessageAuthenticator(s)|
            Attribute::FramedInterfaceId(s)|
            Attribute::FramedIpv6Prefix(s)|
            Attribute::LoginIpv6Host(s)|
            Attribute::FramedIpv6Route(s)|
            Attribute::FramedIpv6Pool(s)|
            Attribute::RouteIpv6Information(s)|
            Attribute::AcctSessionId(s)|
            Attribute::DigestResponse(s)|
            Attribute::DigestAttributes(s)|
            Attribute::DigestRealm(s)|
            Attribute::DigestNonce(s)|
            Attribute::DigestMethod(s)|
            Attribute::DigestUri(s)|
            Attribute::DigestQop(s)|
            Attribute::DigestAlgorithm(s)|
            Attribute::DigestBodyDigest(s)|
            Attribute::DigestCnonce(s)|
            Attribute::DigestNonceCount(s)|
            Attribute::DigestUserName(s)|
            Attribute::UserId(s)|
            Attribute::UserRealm(s) => Some(s),
            _ => None
        }
    }

    fn to_int_value(Self) -> u32 {
        match Self {
            Attribute::NasPort(value)|
            Attribute::FilterId(value)|
            Attribute::FramedMtu(value)|
            Attribute::FramedIpxNetwork(value)|
            Attribute::SessionTimeout(value)|
            Attribute::IdleTimeout(value)|
            Attribute::EventTimestamp(value)|
            Attribute::PortLimit(value)|
            Attribute::AcctDelayTime(value)|
            Attribute::AcctInputOctets(value)|
            Attribute::AcctOutputOctets(value)|
            Attribute::AcctSessionTime(value)|
            Attribute::AcctInputPackets(value)|
            Attribute::AcctOutputPackets(value)|
            Attribute::AcctMultiSessionId(value)|
            Attribute::AcctLinkCount(value)|
            Attribute::AcctInputGigawords(value)|
            Attribute::AcctOutputGigawords(value)| 
            Attribute::LoginPort(value)|
            Attribute::Expiration(value) => Some(value),
            Attribute::NasIpAddress(value)|
            Attribute::FramedIpAddress(value)|
            Attribute::FramedIpNetmask(value)|
            Attribute::LoginIpHost(value) => None, //TODO
            Attribute::ServiceType(value) => Some(value.to_u32()), 
            Attribute::FramedProtocol(value) => Some(value.to_u32()),
            Attribute::FramedRouting(value) => Some(value.to_u32()),
            Attribute::FramedCompression(value) => Some(value.to_u32()),
            Attribute::TerminationAction(value) => Some(value.to_u32()),
            Attribute::NasPortType(value) => Some(value.to_u32()),
            Attribute::AcctStatusType(value) => Some(value.to_u32()),
            Attribute::AcctAuthentic(value) => Some(value.to_u32()),
            Attribute::AcctTerminateCause(value) => Some(value.to_u32()),
        }
    }
}

impl ToPrimitive for Attribute {
    fn to_u64(&self) -> Option<u64> {
        Some(match *self {
            Attribute::UserName(..) => PW_USER_NAME,
            Attribute::UserPassword(..) => PW_USER_PASSWORD,
            Attribute::ChapPassword(..) => PW_CHAP_PASSWORD,
            Attribute::NasIpAddress(..) => PW_NAS_IP_ADDRESS,
            Attribute::NasPort(..) => PW_NAS_PORT,
            Attribute::ServiceType(..) => PW_SERVICE_TYPE,
            _ => return None,
        } as u64)
    }
    fn to_i64(&self) -> Option<i64> {
        self.to_u64().map(|x| x as i64)
    }
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
            (PW_USER_NAME, AttributeType::String) =>
                Attribute::UserName(string_value.to_owned()),
            (PW_USER_PASSWORD, AttributeType::String) =>
                Attribute::UserPassword(string_value.to_owned()),
            (PW_CHAP_PASSWORD, AttributeType::String) =>
                Attribute::ChapPassword(string_value.to_owned()),
            (PW_NAS_IP_ADDRESS, AttributeType::IpAddress) => 
                Attribute::NasIpAddress(attr.lvalue.into()),
            (PW_NAS_PORT, AttributeType::IpAddress) => 
                Attribute::NasPort(attr.lvalue),
            (PW_SERVICE_TYPE, AttributeType::Integer) => 
                Attribute::ServiceType(
                    ServiceType::from_u32(attr.lvalue)
                    .ok_or(ConversionError::ValuePair)?),
            (PW_FRAMED_PROTOCOL, AttributeType::Integer) => 
                Attribute::FramedProtocol(
                    Protocol::from_u32(attr.lvalue)
                    .ok_or(ConversionError::ValuePair)?),
            (PW_FRAMED_IP_ADDRESS, AttributeType::IpAddress) => 
                Attribute::FramedIpAddress(attr.lvalue.into()),
            (PW_FRAMED_IP_NETMASK, AttributeType::IpAddress) => 
                Attribute::FramedIpNetmask(attr.lvalue.into()),
            (PW_FRAMED_ROUTING, AttributeType::Integer) =>
                Attribute::FramedRouting(
                    RoutingValue::from_u32(attr.lvalue)
                    .ok_or(ConversionError::ValuePair)?),

            _ => return Err(ConversionError::ValuePair)
        })
    }
}
