use std::fmt;
use std::io::{Cursor, Read};
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use anyhow::Result;
use serde::Serialize;

use super::cflist::ChMask;
use super::dl_settings::DLSettings;

pub trait Payload<Struct = Self> {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Struct>;
    fn encode(&self) -> Result<Vec<u8>>;
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CID {
    // LoRaWAN
    ResetInd,
    ResetConf,
    LinkCheckReq,
    LinkCheckAns,
    LinkADRReq,
    LinkADRAns,
    DutyCycleReq,
    DutyCycleAns,
    RxParamSetupReq,
    RxParamSetupAns,
    DevStatusReq,
    DevStatusAns,
    NewChannelReq,
    NewChannelAns,
    RxTimingSetupReq,
    RxTimingSetupAns,
    TxParamSetupReq,
    TxParamSetupAns,
    DlChannelReq,
    DlChannelAns,
    RekeyConf,
    RekeyInd,
    ADRParamSetupReq,
    ADRParamSetupAns,
    DeviceTimeReq,
    DeviceTimeAns,
    ForceRejoinReq,
    RejoinParamSetupReq,
    RejoinParamSetupAns,
    PingSlotInfoReq,
    PingSlotInfoAns,
    PingSlotChannelReq,
    PingSlotChannelAns,
    BeaconFreqReq,
    BeaconFreqAns,
    DeviceModeInd,
    DeviceModeConf,
    // Relay
    RelayConfReq,
    RelayConfAns,
    EndDeviceConfReq,
    EndDeviceConfAns,
    FilterListReq,
    FilterListAns,
    UpdateUplinkListReq,
    UpdateUplinkListAns,
    CtrlUplinkListReq,
    CtrlUplinkListAns,
    ConfigureFwdLimitReq,
    ConfigureFwdLimitAns,
    NotifyNewEndDeviceReq,
    // Raw
    Raw,
}

impl fmt::Display for CID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CID {
    pub fn to_u8(&self) -> u8 {
        match self {
            // LoRaWAN
            CID::ResetInd | CID::ResetConf => 0x01,
            CID::LinkCheckReq | CID::LinkCheckAns => 0x02,
            CID::LinkADRReq | CID::LinkADRAns => 0x03,
            CID::DutyCycleReq | CID::DutyCycleAns => 0x04,
            CID::RxParamSetupReq | CID::RxParamSetupAns => 0x05,
            CID::DevStatusReq | CID::DevStatusAns => 0x06,
            CID::NewChannelReq | CID::NewChannelAns => 0x07,
            CID::RxTimingSetupReq | CID::RxTimingSetupAns => 0x08,
            CID::TxParamSetupReq | CID::TxParamSetupAns => 0x09,
            CID::DlChannelReq | CID::DlChannelAns => 0x0a,
            CID::RekeyConf | CID::RekeyInd => 0x0b,
            CID::ADRParamSetupReq | CID::ADRParamSetupAns => 0x0c,
            CID::DeviceTimeReq | CID::DeviceTimeAns => 0x0d,
            CID::ForceRejoinReq => 0x0e,
            CID::RejoinParamSetupReq | CID::RejoinParamSetupAns => 0x0f,
            CID::PingSlotInfoReq | CID::PingSlotInfoAns => 0x10,
            CID::PingSlotChannelReq | CID::PingSlotChannelAns => 0x11,
            CID::BeaconFreqReq | CID::BeaconFreqAns => 0x13, // 0x12 is deprecated
            CID::DeviceModeInd | CID::DeviceModeConf => 0x20,
            // Relay
            CID::RelayConfReq | CID::RelayConfAns => 0x40,
            CID::EndDeviceConfReq | CID::EndDeviceConfAns => 0x41,
            CID::FilterListReq | CID::FilterListAns => 0x42,
            CID::UpdateUplinkListReq | CID::UpdateUplinkListAns => 0x43,
            CID::CtrlUplinkListReq | CID::CtrlUplinkListAns => 0x44,
            CID::ConfigureFwdLimitReq | CID::ConfigureFwdLimitAns => 0x45,
            CID::NotifyNewEndDeviceReq => 0x46,
            // Raw
            CID::Raw => 0xff,
        }
    }

    pub fn from_u8(uplink: bool, v: u8) -> Result<Self> {
        Ok(if uplink {
            match v {
                0x01 => CID::ResetInd,
                0x02 => CID::LinkCheckReq,
                0x03 => CID::LinkADRAns,
                0x04 => CID::DutyCycleAns,
                0x05 => CID::RxParamSetupAns,
                0x06 => CID::DevStatusAns,
                0x07 => CID::NewChannelAns,
                0x08 => CID::RxTimingSetupAns,
                0x09 => CID::TxParamSetupAns,
                0x0a => CID::DlChannelAns,
                0x0b => CID::RekeyInd,
                0x0c => CID::ADRParamSetupAns,
                0x0d => CID::DeviceTimeReq,
                0x0f => CID::RejoinParamSetupAns,
                0x10 => CID::PingSlotInfoReq,
                0x11 => CID::PingSlotChannelAns,
                0x13 => CID::BeaconFreqAns,
                0x20 => CID::DeviceModeInd,
                _ => {
                    return Err(anyhow!("Invalid CID: {}", v));
                }
            }
        } else {
            match v {
                0x01 => CID::ResetConf,
                0x02 => CID::LinkCheckAns,
                0x03 => CID::LinkADRReq,
                0x04 => CID::DutyCycleReq,
                0x05 => CID::RxParamSetupReq,
                0x06 => CID::DevStatusReq,
                0x07 => CID::NewChannelReq,
                0x08 => CID::RxTimingSetupReq,
                0x09 => CID::TxParamSetupReq,
                0x0a => CID::DlChannelReq,
                0x0b => CID::RekeyConf,
                0x0c => CID::ADRParamSetupReq,
                0x0d => CID::DeviceTimeAns,
                0x0e => CID::ForceRejoinReq,
                0x0f => CID::RejoinParamSetupReq,
                0x10 => CID::PingSlotInfoAns,
                0x11 => CID::PingSlotChannelReq,
                0x13 => CID::BeaconFreqReq,
                0x20 => CID::DeviceModeConf,
                _ => {
                    return Err(anyhow!("Invalid CID: {}", v));
                }
            }
        })
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum MACCommand {
    // LoRaWAN
    ResetInd(ResetIndPayload),
    ResetConf(ResetConfPayload),
    LinkCheckReq,
    LinkCheckAns(LinkCheckAnsPayload),
    LinkADRReq(LinkADRReqPayload),
    LinkADRAns(LinkADRAnsPayload),
    DutyCycleReq(DutyCycleReqPayload),
    DutyCycleAns,
    RxParamSetupReq(RxParamSetupReqPayload),
    RxParamSetupAns(RxParamSetupAnsPayload),
    DevStatusReq,
    DevStatusAns(DevStatusAnsPayload),
    NewChannelReq(NewChannelReqPayload),
    NewChannelAns(NewChannelAnsPayload),
    RxTimingSetupReq(RxTimingSetupReqPayload),
    RxTimingSetupAns,
    TxParamSetupReq(TxParamSetupReqPayload),
    TxParamSetupAns,
    DlChannelReq(DlChannelReqPayload),
    DlChannelAns(DlChannelAnsPayload),
    RekeyConf(RekeyConfPayload),
    RekeyInd(RekeyIndPayload),
    ADRParamSetupReq(ADRParamSetupReqPayload),
    ADRParamSetupAns,
    DeviceTimeReq,
    DeviceTimeAns(DeviceTimeAnsPayload),
    ForceRejoinReq(ForceRejoinReqPayload),
    RejoinParamSetupReq(RejoinParamSetupReqPayload),
    RejoinParamSetupAns(RejoinParamSetupAnsPayload),
    PingSlotInfoReq(PingSlotInfoReqPayload),
    PingSlotInfoAns,
    PingSlotChannelReq(PingSlotChannelReqPayload),
    PingSlotChannelAns(PingSlotChannelAnsPayload),
    BeaconFreqReq(BeaconFreqReqPayload),
    BeaconFreqAns(BeaconFreqAnsPayload),
    DeviceModeInd(DeviceModeIndPayload),
    DeviceModeConf(DeviceModeConfPayload),
    // Relay
    RelayConfReq(RelayConfReqPayload),
    RelayConfAns(RelayConfAnsPayload),
    EndDeviceConfReq(EndDeviceConfReqPayload),
    EndDeviceConfAns(EndDeviceConfAnsPayload),
    // Raw
    Raw(Vec<u8>),
}

impl MACCommand {
    pub fn cid(&self) -> CID {
        match self {
            // LoRaWAN
            MACCommand::ResetInd(_) => CID::ResetInd,
            MACCommand::ResetConf(_) => CID::ResetConf,
            MACCommand::LinkCheckReq => CID::LinkCheckReq,
            MACCommand::LinkCheckAns(_) => CID::LinkCheckAns,
            MACCommand::LinkADRReq(_) => CID::LinkADRReq,
            MACCommand::LinkADRAns(_) => CID::LinkADRAns,
            MACCommand::DutyCycleReq(_) => CID::DutyCycleReq,
            MACCommand::DutyCycleAns => CID::DutyCycleAns,
            MACCommand::RxParamSetupReq(_) => CID::RxParamSetupReq,
            MACCommand::RxParamSetupAns(_) => CID::RxParamSetupAns,
            MACCommand::DevStatusReq => CID::DevStatusReq,
            MACCommand::DevStatusAns(_) => CID::DevStatusAns,
            MACCommand::NewChannelReq(_) => CID::NewChannelReq,
            MACCommand::NewChannelAns(_) => CID::NewChannelAns,
            MACCommand::RxTimingSetupReq(_) => CID::RxTimingSetupReq,
            MACCommand::RxTimingSetupAns => CID::RxTimingSetupAns,
            MACCommand::TxParamSetupReq(_) => CID::TxParamSetupReq,
            MACCommand::TxParamSetupAns => CID::TxParamSetupAns,
            MACCommand::DlChannelReq(_) => CID::DlChannelReq,
            MACCommand::DlChannelAns(_) => CID::DlChannelAns,
            MACCommand::RekeyConf(_) => CID::RekeyConf,
            MACCommand::RekeyInd(_) => CID::RekeyInd,
            MACCommand::ADRParamSetupReq(_) => CID::ADRParamSetupReq,
            MACCommand::ADRParamSetupAns => CID::ADRParamSetupAns,
            MACCommand::DeviceTimeReq => CID::DeviceTimeReq,
            MACCommand::DeviceTimeAns(_) => CID::DeviceTimeAns,
            MACCommand::ForceRejoinReq(_) => CID::ForceRejoinReq,
            MACCommand::RejoinParamSetupReq(_) => CID::RejoinParamSetupReq,
            MACCommand::RejoinParamSetupAns(_) => CID::RejoinParamSetupAns,
            MACCommand::PingSlotInfoReq(_) => CID::PingSlotInfoReq,
            MACCommand::PingSlotInfoAns => CID::PingSlotInfoAns,
            MACCommand::PingSlotChannelReq(_) => CID::PingSlotChannelReq,
            MACCommand::PingSlotChannelAns(_) => CID::PingSlotChannelAns,
            MACCommand::BeaconFreqReq(_) => CID::BeaconFreqReq,
            MACCommand::BeaconFreqAns(_) => CID::BeaconFreqAns, // 0x12 is deprecated
            MACCommand::DeviceModeInd(_) => CID::DeviceModeInd,
            MACCommand::DeviceModeConf(_) => CID::DeviceModeConf,
            // Relay
            MACCommand::RelayConfReq(_) => CID::RelayConfReq,
            MACCommand::RelayConfAns(_) => CID::RelayConfAns,
            MACCommand::EndDeviceConfReq(_) => CID::EndDeviceConfReq,
            MACCommand::EndDeviceConfAns(_) => CID::EndDeviceConfAns,
            // Raw
            MACCommand::Raw(_) => CID::Raw,
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Version {
    LoRaWAN1_1,
}

impl Version {
    pub fn to_u8(&self) -> u8 {
        match self {
            Version::LoRaWAN1_1 => 0x01,
        }
    }

    pub fn from_u8(v: u8) -> Result<Self> {
        Ok(match v {
            0x01 => Version::LoRaWAN1_1,
            _ => {
                return Err(anyhow!("invalid version"));
            }
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DwellTime {
    NoLimit,
    Limit400ms,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeviceModeClass {
    ClassA,
    ClassC,
}

impl DeviceModeClass {
    pub fn to_u8(&self) -> u8 {
        match self {
            DeviceModeClass::ClassA => 0x00,
            DeviceModeClass::ClassC => 0x02,
        }
    }

    pub fn from_u8(v: u8) -> Result<Self> {
        Ok(match v {
            0x00 => DeviceModeClass::ClassA,
            0x02 => DeviceModeClass::ClassC,
            _ => {
                return Err(anyhow!("invalid device mode"));
            }
        })
    }
}

impl fmt::Display for DeviceModeClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DeviceModeClass::ClassA => "A",
                DeviceModeClass::ClassC => "C",
            }
        )
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct MACCommandSet(Vec<MACCommand>);

impl Deref for MACCommandSet {
    type Target = Vec<MACCommand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MACCommandSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MACCommandSet {
    pub fn new(macs: Vec<MACCommand>) -> Self {
        MACCommandSet(macs)
    }

    pub fn size(&self) -> Result<usize> {
        let b = self.to_vec()?;
        Ok(b.len())
    }

    pub fn from_slice(b: &[u8]) -> Self {
        // For LoRaWAN 1.1, this payload must be first decrypted before it can be parsed.
        // Therefore, the decoding into separate mac-commands is handled by decode_from_raw.
        MACCommandSet(vec![MACCommand::Raw(b.to_vec())])
    }

    pub fn push(&mut self, m: MACCommand) {
        self.0.push(m);
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut out = Vec::new();

        for mac in &self.0 {
            match mac {
                // LoRaWAN
                MACCommand::ResetInd(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::ResetConf(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::LinkCheckReq => {
                    out.push(mac.cid().to_u8());
                }
                MACCommand::LinkCheckAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::LinkADRReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::LinkADRAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::DutyCycleReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::DutyCycleAns => {
                    out.push(mac.cid().to_u8());
                }
                MACCommand::RxParamSetupReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::RxParamSetupAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::DevStatusReq => {
                    out.push(mac.cid().to_u8());
                }
                MACCommand::DevStatusAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::NewChannelReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::NewChannelAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::RxTimingSetupReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::RxTimingSetupAns => {
                    out.push(mac.cid().to_u8());
                }
                MACCommand::TxParamSetupReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::TxParamSetupAns => {
                    out.push(mac.cid().to_u8());
                }
                MACCommand::DlChannelReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::DlChannelAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::RekeyConf(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::RekeyInd(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::ADRParamSetupReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::ADRParamSetupAns => {
                    out.push(mac.cid().to_u8());
                }
                MACCommand::DeviceTimeReq => {
                    out.push(mac.cid().to_u8());
                }
                MACCommand::DeviceTimeAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::ForceRejoinReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::RejoinParamSetupReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::RejoinParamSetupAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::PingSlotInfoReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::PingSlotInfoAns => {
                    out.push(mac.cid().to_u8());
                }
                MACCommand::PingSlotChannelReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::PingSlotChannelAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::BeaconFreqReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::BeaconFreqAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::DeviceModeInd(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::DeviceModeConf(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                // Relay
                MACCommand::RelayConfReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::RelayConfAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::EndDeviceConfReq(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                MACCommand::EndDeviceConfAns(pl) => {
                    out.push(mac.cid().to_u8());
                    out.extend_from_slice(&pl.encode()?);
                }
                // Raw
                MACCommand::Raw(v) => out.extend_from_slice(v),
            };
        }

        Ok(out)
    }

    pub fn decode_from_raw(&mut self, uplink: bool) -> Result<()> {
        // nothing to parse
        if self.0.is_empty() {
            return Ok(());
        }

        // in any other case there must be exactly one MACCommand::Raw.
        if self.0.len() == 1 {
            if let MACCommand::Raw(b) = &self.0[0] {
                let mut cur = Cursor::new(b.clone());
                let mut commands = vec![];
                let mut b = [0; 1];

                loop {
                    // Try to read one byte to get the CID.
                    if cur.read_exact(&mut b).is_err() {
                        break;
                    }

                    let cid = match CID::from_u8(uplink, b[0]) {
                        Ok(v) => v,
                        Err(_) => {
                            let mut b = b.to_vec();
                            cur.read_to_end(&mut b)?;
                            commands.push(MACCommand::Raw(b));
                            break;
                        }
                    };

                    match cid {
                        CID::ResetInd => {
                            commands.push(MACCommand::ResetInd(ResetIndPayload::decode(&mut cur)?))
                        }
                        CID::ResetConf => commands
                            .push(MACCommand::ResetConf(ResetConfPayload::decode(&mut cur)?)),
                        CID::LinkCheckReq => commands.push(MACCommand::LinkCheckReq),
                        CID::LinkCheckAns => commands.push(MACCommand::LinkCheckAns(
                            LinkCheckAnsPayload::decode(&mut cur)?,
                        )),
                        CID::LinkADRReq => commands
                            .push(MACCommand::LinkADRReq(LinkADRReqPayload::decode(&mut cur)?)),
                        CID::LinkADRAns => commands
                            .push(MACCommand::LinkADRAns(LinkADRAnsPayload::decode(&mut cur)?)),
                        CID::DutyCycleReq => commands.push(MACCommand::DutyCycleReq(
                            DutyCycleReqPayload::decode(&mut cur)?,
                        )),
                        CID::DutyCycleAns => commands.push(MACCommand::DutyCycleAns),
                        CID::RxParamSetupReq => commands.push(MACCommand::RxParamSetupReq(
                            RxParamSetupReqPayload::decode(&mut cur)?,
                        )),
                        CID::RxParamSetupAns => commands.push(MACCommand::RxParamSetupAns(
                            RxParamSetupAnsPayload::decode(&mut cur)?,
                        )),
                        CID::DevStatusReq => commands.push(MACCommand::DevStatusReq),
                        CID::DevStatusAns => commands.push(MACCommand::DevStatusAns(
                            DevStatusAnsPayload::decode(&mut cur)?,
                        )),
                        CID::NewChannelReq => commands.push(MACCommand::NewChannelReq(
                            NewChannelReqPayload::decode(&mut cur)?,
                        )),
                        CID::NewChannelAns => commands.push(MACCommand::NewChannelAns(
                            NewChannelAnsPayload::decode(&mut cur)?,
                        )),
                        CID::RxTimingSetupReq => commands.push(MACCommand::RxTimingSetupReq(
                            RxTimingSetupReqPayload::decode(&mut cur)?,
                        )),
                        CID::RxTimingSetupAns => commands.push(MACCommand::RxTimingSetupAns),
                        CID::TxParamSetupReq => commands.push(MACCommand::TxParamSetupReq(
                            TxParamSetupReqPayload::decode(&mut cur)?,
                        )),
                        CID::TxParamSetupAns => commands.push(MACCommand::TxParamSetupAns),
                        CID::DlChannelReq => commands.push(MACCommand::DlChannelReq(
                            DlChannelReqPayload::decode(&mut cur)?,
                        )),
                        CID::DlChannelAns => commands.push(MACCommand::DlChannelAns(
                            DlChannelAnsPayload::decode(&mut cur)?,
                        )),
                        CID::RekeyConf => commands
                            .push(MACCommand::RekeyConf(RekeyConfPayload::decode(&mut cur)?)),
                        CID::RekeyInd => {
                            commands.push(MACCommand::RekeyInd(RekeyIndPayload::decode(&mut cur)?))
                        }
                        CID::ADRParamSetupReq => commands.push(MACCommand::ADRParamSetupReq(
                            ADRParamSetupReqPayload::decode(&mut cur)?,
                        )),
                        CID::ADRParamSetupAns => commands.push(MACCommand::ADRParamSetupAns),
                        CID::DeviceTimeReq => commands.push(MACCommand::DeviceTimeReq),
                        CID::DeviceTimeAns => commands.push(MACCommand::DeviceTimeAns(
                            DeviceTimeAnsPayload::decode(&mut cur)?,
                        )),
                        CID::ForceRejoinReq => commands.push(MACCommand::ForceRejoinReq(
                            ForceRejoinReqPayload::decode(&mut cur)?,
                        )),
                        CID::RejoinParamSetupReq => commands.push(MACCommand::RejoinParamSetupReq(
                            RejoinParamSetupReqPayload::decode(&mut cur)?,
                        )),
                        CID::RejoinParamSetupAns => commands.push(MACCommand::RejoinParamSetupAns(
                            RejoinParamSetupAnsPayload::decode(&mut cur)?,
                        )),
                        CID::PingSlotInfoReq => commands.push(MACCommand::PingSlotInfoReq(
                            PingSlotInfoReqPayload::decode(&mut cur)?,
                        )),
                        CID::PingSlotInfoAns => commands.push(MACCommand::PingSlotInfoAns),
                        CID::PingSlotChannelReq => commands.push(MACCommand::PingSlotChannelReq(
                            PingSlotChannelReqPayload::decode(&mut cur)?,
                        )),
                        CID::PingSlotChannelAns => commands.push(MACCommand::PingSlotChannelAns(
                            PingSlotChannelAnsPayload::decode(&mut cur)?,
                        )),
                        CID::BeaconFreqReq => commands.push(MACCommand::BeaconFreqReq(
                            BeaconFreqReqPayload::decode(&mut cur)?,
                        )),
                        CID::BeaconFreqAns => commands.push(MACCommand::BeaconFreqAns(
                            BeaconFreqAnsPayload::decode(&mut cur)?,
                        )),
                        CID::DeviceModeInd => commands.push(MACCommand::DeviceModeInd(
                            DeviceModeIndPayload::decode(&mut cur)?,
                        )),
                        CID::DeviceModeConf => commands.push(MACCommand::DeviceModeConf(
                            DeviceModeConfPayload::decode(&mut cur)?,
                        )),
                        CID::RelayConfReq => commands.push(MACCommand::RelayConfReq(
                            RelayConfReqPayload::decode(&mut cur)?,
                        )),
                        CID::RelayConfAns => commands.push(MACCommand::RelayConfAns(
                            RelayConfAnsPayload::decode(&mut cur)?,
                        )),
                        CID::EndDeviceConfReq => commands.push(MACCommand::EndDeviceConfReq(
                            EndDeviceConfReqPayload::decode(&mut cur)?,
                        )),
                        CID::EndDeviceConfAns => commands.push(MACCommand::EndDeviceConfAns(
                            EndDeviceConfAnsPayload::decode(&mut cur)?,
                        )),
                        _ => todo!(),
                    }
                }

                // Overwrite with decoded mac-commands.
                self.0 = commands;
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ResetIndPayload {
    pub dev_lorawan_version: Version,
}

impl Payload for ResetIndPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(ResetIndPayload {
            dev_lorawan_version: Version::from_u8(b[0])?,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![self.dev_lorawan_version.to_u8()])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ResetConfPayload {
    pub serv_lorawan_version: Version,
}

impl Payload for ResetConfPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(ResetConfPayload {
            serv_lorawan_version: Version::from_u8(b[0])?,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![self.serv_lorawan_version.to_u8()])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct LinkCheckAnsPayload {
    pub margin: u8,
    pub gw_cnt: u8,
}

impl Payload for LinkCheckAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 2];
        cur.read_exact(&mut b)?;

        Ok(LinkCheckAnsPayload {
            margin: b[0],
            gw_cnt: b[1],
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![self.margin, self.gw_cnt])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct LinkADRReqPayload {
    pub dr: u8,
    pub tx_power: u8,
    pub ch_mask: ChMask,
    pub redundancy: Redundancy,
}

impl Payload for LinkADRReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 4];
        cur.read_exact(&mut b)?;

        Ok(LinkADRReqPayload {
            dr: (b[0] & 0xf0) >> 4,
            tx_power: b[0] & 0x0f,
            ch_mask: ChMask::from_bytes([b[1], b[2]]),
            redundancy: Redundancy::from_u8(b[3]),
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b = vec![0; 4];

        if self.dr > 15 {
            return Err(anyhow!("max value of dr is 15"));
        }

        if self.tx_power > 15 {
            return Err(anyhow!("max value of tx_power is 15"));
        }

        b[0] = self.tx_power | (self.dr << 4);
        b[1..3].clone_from_slice(&self.ch_mask.to_bytes());
        b[3] = self.redundancy.to_u8()?;

        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Redundancy {
    pub ch_mask_cntl: u8,
    pub nb_rep: u8,
}

impl Redundancy {
    pub fn to_u8(&self) -> Result<u8> {
        if self.nb_rep > 15 {
            return Err(anyhow!("max value of nb_rep is 15"));
        }
        if self.ch_mask_cntl > 7 {
            return Err(anyhow!("max value of ch_mask_cntl is 7"));
        }

        Ok(self.nb_rep | (self.ch_mask_cntl << 4))
    }

    pub fn from_u8(b: u8) -> Self {
        Redundancy {
            nb_rep: b & 0x0f,
            ch_mask_cntl: (b & 0x70) >> 4,
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct LinkADRAnsPayload {
    pub ch_mask_ack: bool,
    pub dr_ack: bool,
    pub tx_power_ack: bool,
}

impl Payload for LinkADRAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(LinkADRAnsPayload {
            ch_mask_ack: b[0] & 0x01 != 0,
            dr_ack: b[0] & 0x02 != 0,
            tx_power_ack: b[0] & 0x04 != 0,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b: u8 = 0;

        if self.ch_mask_ack {
            b |= 0x01;
        }
        if self.dr_ack {
            b |= 0x02;
        }
        if self.tx_power_ack {
            b |= 0x04;
        }

        Ok(vec![b])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct DutyCycleReqPayload {
    pub max_duty_cycle: u8,
}

impl Payload for DutyCycleReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(DutyCycleReqPayload {
            max_duty_cycle: b[0],
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.max_duty_cycle > 15 && self.max_duty_cycle != 255 {
            return Err(anyhow!("max_duty_cycle must have value 0 - 15 or 255"));
        }

        Ok(vec![self.max_duty_cycle])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RxParamSetupReqPayload {
    pub frequency: u32,
    pub dl_settings: DLSettings,
}

impl Payload for RxParamSetupReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 4];
        cur.read_exact(&mut b)?;

        // TODO: check for 2.4 GHz frequency compatibility
        Ok(RxParamSetupReqPayload {
            dl_settings: DLSettings::from_le_bytes([b[0]]),
            frequency: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(&b[1..]);
                u32::from_le_bytes(freq_b) * 100
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        // TODO: check for 2.4 GHz frequency compatibility
        if self.frequency / 100 >= (1 << 24) {
            return Err(anyhow!("max frequency value is 2^24-1"));
        }
        if self.frequency % 100 != 0 {
            return Err(anyhow!("frequency must be a multiple of 100"));
        }

        let mut b = vec![0; 4];

        b[0..1].copy_from_slice(&self.dl_settings.to_le_bytes()?);

        let freq_b = (self.frequency / 100).to_le_bytes();
        b[1..4].copy_from_slice(&freq_b[0..3]);
        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RxParamSetupAnsPayload {
    pub channel_ack: bool,
    pub rx2_dr_ack: bool,
    pub rx1_dr_offset_ack: bool,
}

impl Payload for RxParamSetupAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(RxParamSetupAnsPayload {
            channel_ack: b[0] & 0x01 != 0,
            rx2_dr_ack: b[0] & 0x02 != 0,
            rx1_dr_offset_ack: b[0] & 0x04 != 0,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b: u8 = 0;
        if self.channel_ack {
            b |= 0x01;
        }
        if self.rx2_dr_ack {
            b |= 0x02;
        }
        if self.rx1_dr_offset_ack {
            b |= 0x04;
        }
        Ok(vec![b])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct DevStatusAnsPayload {
    pub battery: u8,
    pub margin: i8,
}

impl Payload for DevStatusAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 2];
        cur.read_exact(&mut b)?;

        Ok(DevStatusAnsPayload {
            battery: b[0],
            margin: {
                if b[1] > 31 {
                    (b[1] as i8) - 64
                } else {
                    b[1] as i8
                }
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.margin < -32 {
            return Err(anyhow!("min margin value is -32"));
        }
        if self.margin > 31 {
            return Err(anyhow!("max margin value is 31"));
        }

        Ok(vec![self.battery, {
            if self.margin < 0 {
                (self.margin + 64) as u8
            } else {
                self.margin as u8
            }
        }])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct NewChannelReqPayload {
    pub ch_index: u8,
    pub freq: u32,
    pub min_dr: u8,
    pub max_dr: u8,
}

impl Payload for NewChannelReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 5];
        cur.read_exact(&mut b)?;

        Ok(NewChannelReqPayload {
            ch_index: b[0],
            freq: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(&b[1..4]);
                let freq = u32::from_le_bytes(freq_b);

                if freq >= 12000000 {
                    // 2.4GHz frequency
                    freq * 200
                } else {
                    freq * 100
                }
            },
            min_dr: b[4] & 0x0f,
            max_dr: (b[4] & 0xf0) >> 4,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut freq = self.freq;

        // Support LoRaWAN 2.4GHz, in which case the stepping is 200Hz:
        // See Frequency Encoding in MAC Commands
        // https://lora-developers.semtech.com/documentation/tech-papers-and-guides/physical-layer-proposal-2.4ghz/
        if freq >= 2400000000 {
            freq /= 2;
        }

        if freq / 100 >= (1 << 24) {
            return Err(anyhow!("max freq value is 2^24 - 1"));
        }
        if freq % 100 != 0 {
            return Err(anyhow!("freq must be multiple of 100"));
        }
        if self.min_dr > 15 {
            return Err(anyhow!("max min_dr value is 15"));
        }
        if self.max_dr > 15 {
            return Err(anyhow!("max max_dr value is 15"));
        }

        let mut b = vec![0; 5];
        b[0] = self.ch_index;
        b[1..5].copy_from_slice(&(freq / 100).to_le_bytes());
        b[4] = self.min_dr | (self.max_dr << 4);

        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct NewChannelAnsPayload {
    pub channel_freq_ok: bool,
    pub dr_range_ok: bool,
}

impl Payload for NewChannelAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(NewChannelAnsPayload {
            channel_freq_ok: b[0] & 0x01 != 0,
            dr_range_ok: b[0] & 0x02 != 0,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b: u8 = 0;
        if self.channel_freq_ok {
            b = 0x01;
        }
        if self.dr_range_ok {
            b |= 0x02;
        }
        Ok(vec![b])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RxTimingSetupReqPayload {
    pub delay: u8,
}

impl Payload for RxTimingSetupReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;
        Ok(RxTimingSetupReqPayload { delay: b[0] })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.delay > 15 {
            return Err(anyhow!("max delay value is 15"));
        }

        Ok(vec![self.delay])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct TxParamSetupReqPayload {
    pub uplink_dwell_time: DwellTime,
    pub downlink_dwell_time: DwellTime,
    pub max_eirp: u8,
}

impl Payload for TxParamSetupReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(TxParamSetupReqPayload {
            uplink_dwell_time: {
                if b[0] & 0x10 != 0 {
                    DwellTime::Limit400ms
                } else {
                    DwellTime::NoLimit
                }
            },
            downlink_dwell_time: {
                if b[0] & 0x20 != 0 {
                    DwellTime::Limit400ms
                } else {
                    DwellTime::NoLimit
                }
            },
            max_eirp: b[0] & 0x0f,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.max_eirp > 15 {
            return Err(anyhow!("max max_eirp value is 15"));
        }

        let mut b = vec![self.max_eirp];
        if self.uplink_dwell_time == DwellTime::Limit400ms {
            b[0] |= 0x10;
        }
        if self.downlink_dwell_time == DwellTime::Limit400ms {
            b[0] |= 0x20;
        }

        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct DlChannelReqPayload {
    pub ch_index: u8,
    pub freq: u32,
}

impl Payload for DlChannelReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 4];
        cur.read_exact(&mut b)?;

        // TODO: check for 2.4 GHz frequency compatibility
        Ok(DlChannelReqPayload {
            ch_index: b[0],
            freq: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(&b[1..4]);
                u32::from_le_bytes(freq_b) * 100
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        // TODO: check for 2.4 GHz frequency compatibility
        if self.freq / 100 >= 1 << 24 {
            return Err(anyhow!("max freq value is 2^24 - 1"));
        }
        if self.freq % 100 != 0 {
            return Err(anyhow!("freq must be a multiple of 100"));
        }

        let mut b = vec![0; 4];
        b[0] = self.ch_index;

        let freq_b = (self.freq / 100).to_le_bytes();
        b[1..4].copy_from_slice(&freq_b[0..3]);

        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct DlChannelAnsPayload {
    pub uplink_freq_exists: bool,
    pub channel_freq_ok: bool,
}

impl Payload for DlChannelAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(DlChannelAnsPayload {
            channel_freq_ok: b[0] & 0x01 != 0,
            uplink_freq_exists: b[0] & 0x02 != 0,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b: u8 = 0;

        if self.channel_freq_ok {
            b |= 0x01;
        }
        if self.uplink_freq_exists {
            b |= 0x02;
        }

        Ok(vec![b])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RekeyConfPayload {
    pub serv_lorawan_version: Version,
}

impl Payload for RekeyConfPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(RekeyConfPayload {
            serv_lorawan_version: Version::from_u8(b[0])?,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![self.serv_lorawan_version.to_u8()])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RekeyIndPayload {
    pub dev_lorawan_version: Version,
}

impl Payload for RekeyIndPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(RekeyIndPayload {
            dev_lorawan_version: Version::from_u8(b[0])?,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![self.dev_lorawan_version.to_u8()])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ADRParamSetupReqPayload {
    pub adr_param: ADRParam,
}

impl Payload for ADRParamSetupReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(ADRParamSetupReqPayload {
            adr_param: ADRParam::from_u8(b[0]),
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![self.adr_param.to_u8()?])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ADRParam {
    pub limit_exp: u8,
    pub delay_exp: u8,
}

impl ADRParam {
    pub fn from_u8(b: u8) -> Self {
        ADRParam {
            delay_exp: b & 0x0f,
            limit_exp: b >> 4,
        }
    }

    pub fn to_u8(&self) -> Result<u8> {
        if self.limit_exp > 15 {
            return Err(anyhow!("max limit_exp value is 15"));
        }
        if self.delay_exp > 15 {
            return Err(anyhow!("max delay_exp value is 15"));
        }

        Ok(self.delay_exp | (self.limit_exp << 4))
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct DeviceTimeAnsPayload {
    pub time_since_gps_epoch: Duration,
}

impl Payload for DeviceTimeAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 5];
        cur.read_exact(&mut b)?;

        let secs = {
            let mut secs_b: [u8; 4] = [0; 4];
            secs_b.copy_from_slice(&b[0..4]);
            u32::from_le_bytes(secs_b)
        } as u64;

        let nanos = (b[4] as u32) * 3906250; // second / 256

        Ok(DeviceTimeAnsPayload {
            time_since_gps_epoch: Duration::new(secs, nanos),
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b = vec![0; 5];
        b[0..4].copy_from_slice(&(self.time_since_gps_epoch.as_secs() as u32).to_le_bytes());
        b[4] = ((self.time_since_gps_epoch.as_nanos() % 1_000_000_000) / 3906250) as u8;
        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ForceRejoinReqPayload {
    pub period: u8,
    pub max_retries: u8,
    pub rejoin_type: u8,
    pub dr: u8,
}

impl Payload for ForceRejoinReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 2];
        cur.read_exact(&mut b)?;

        Ok(ForceRejoinReqPayload {
            dr: b[0] & 0x0f,
            rejoin_type: (b[0] & 0x70) >> 4,
            max_retries: b[1] & 0x07,
            period: (b[1] & 0x38) >> 3,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.period > 7 {
            return Err(anyhow!("max period value is 7"));
        }
        if self.max_retries > 7 {
            return Err(anyhow!("max max_retries value is 7"));
        }
        if self.rejoin_type != 0 && self.rejoin_type != 2 {
            return Err(anyhow!("rejoin_type must be 0 or 2"));
        }
        if self.dr > 15 {
            return Err(anyhow!("max dr value is 15"));
        }

        Ok(vec![
            self.dr | (self.rejoin_type << 4),
            self.max_retries | (self.period << 3),
        ])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RejoinParamSetupReqPayload {
    pub max_time_n: u8,
    pub max_count_n: u8,
}

impl Payload for RejoinParamSetupReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(RejoinParamSetupReqPayload {
            max_count_n: b[0] & 0x0f,
            max_time_n: (b[0] & 0xf0) >> 4,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.max_time_n > 15 {
            return Err(anyhow!("max max_time_n value is 15"));
        }
        if self.max_count_n > 15 {
            return Err(anyhow!("max max_count_n value is 15"));
        }

        Ok(vec![self.max_count_n | (self.max_time_n << 4)])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RejoinParamSetupAnsPayload {
    pub time_ok: bool,
}

impl Payload for RejoinParamSetupAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(RejoinParamSetupAnsPayload {
            time_ok: b[0] & 0x01 != 0,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b: u8 = 0;
        if self.time_ok {
            b = 0x01;
        }
        Ok(vec![b])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct PingSlotInfoReqPayload {
    pub periodicity: u8,
}

impl Payload for PingSlotInfoReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(PingSlotInfoReqPayload {
            periodicity: b[0] & 0x07,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.periodicity > 7 {
            return Err(anyhow!("max periodicity value is 7"));
        }

        Ok(vec![self.periodicity])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct PingSlotChannelReqPayload {
    pub freq: u32,
    pub dr: u8,
}

impl Payload for PingSlotChannelReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 4];
        cur.read_exact(&mut b)?;

        // TODO: check 2.4 GHz frequency compatibility
        Ok(PingSlotChannelReqPayload {
            freq: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(&b[0..3]);
                u32::from_le_bytes(freq_b) * 100
            },
            dr: b[3] & 0x0f,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.freq / 100 >= 1 << 24 {
            return Err(anyhow!("max freq value is 2^24 - 1"));
        }
        if self.freq % 100 != 0 {
            return Err(anyhow!("freq must be a multiple of 100"));
        }
        if self.dr > 15 {
            return Err(anyhow!("max dr value is 15"));
        }

        let mut b = (self.freq / 100).to_le_bytes();
        b[3] = self.dr;

        Ok(b.to_vec())
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct PingSlotChannelAnsPayload {
    pub dr_ok: bool,
    pub channel_freq_ok: bool,
}

impl Payload for PingSlotChannelAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(PingSlotChannelAnsPayload {
            channel_freq_ok: b[0] & 0x01 != 0,
            dr_ok: b[0] & 0x02 != 0,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b = 0;
        if self.channel_freq_ok {
            b = 0x01;
        }
        if self.dr_ok {
            b |= 0x02;
        }
        Ok(vec![b])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct BeaconFreqReqPayload {
    pub freq: u32,
}

impl Payload for BeaconFreqReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 3];
        cur.read_exact(&mut b)?;

        // TODO: check for 2.4GHz frequency compatibility
        Ok(BeaconFreqReqPayload {
            freq: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(&b);
                u32::from_le_bytes(freq_b) * 100
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.freq / 100 >= 1 << 24 {
            return Err(anyhow!("max freq value is 2^24 - 1"));
        }
        if self.freq % 100 != 0 {
            return Err(anyhow!("freq must be a multiple of 100"));
        }

        let freq_b = (self.freq / 100).to_le_bytes();
        let mut b = vec![0; 3];
        b[0..3].copy_from_slice(&freq_b[0..3]);
        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct BeaconFreqAnsPayload {
    beacon_freq_ok: bool,
}

impl Payload for BeaconFreqAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(BeaconFreqAnsPayload {
            beacon_freq_ok: b[0] & 0x01 != 0,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b: u8 = 0;
        if self.beacon_freq_ok {
            b = 0x01;
        }
        Ok(vec![b])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct DeviceModeIndPayload {
    pub class: DeviceModeClass,
}

impl Payload for DeviceModeIndPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(DeviceModeIndPayload {
            class: DeviceModeClass::from_u8(b[0])?,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![self.class.to_u8()])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct DeviceModeConfPayload {
    pub class: DeviceModeClass,
}

impl Payload for DeviceModeConfPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(DeviceModeConfPayload {
            class: DeviceModeClass::from_u8(b[0])?,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![self.class.to_u8()])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ChannelSettingsRelay {
    pub start_stop: u8,
    pub cad_periodicity: u8,
    pub default_ch_idx: u8,
    pub second_ch_idx: u8,
    pub second_ch_dr: u8,
    pub second_ch_ack_offset: u8,
}

impl ChannelSettingsRelay {
    const SIZE: usize = 2;

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.start_stop > 1 {
            return Err(anyhow!("max value of start_stop is 1"));
        }
        if self.cad_periodicity > 7 {
            return Err(anyhow!("max value of cad_periodicity is 7"));
        }
        if self.default_ch_idx > 1 {
            return Err(anyhow!("max value of default_ch_idx is 1"));
        }
        if self.second_ch_idx > 1 {
            return Err(anyhow!("max value of second_ch_idx is 1"));
        }
        if self.second_ch_dr > 15 {
            return Err(anyhow!("max value of second_ch_dr is 15"));
        }
        if self.second_ch_ack_offset > 7 {
            return Err(anyhow!("max value of second_ch_ack_offset is 7"));
        }

        Ok([
            self.second_ch_ack_offset | (self.second_ch_dr << 3) | (self.second_ch_idx << 7),
            (self.second_ch_idx >> 1)
                | (self.default_ch_idx << 1)
                | (self.cad_periodicity << 2)
                | (self.start_stop << 5),
        ])
    }

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("ChannelSettingsRelay expects 2 bytes"));
        }

        Ok(ChannelSettingsRelay {
            second_ch_ack_offset: b[0] & 0x07,
            second_ch_dr: (b[0] & 0x78) >> 3,
            second_ch_idx: (b[0] & 0x80) >> 7 | (b[1] & 0x01) << 1,
            default_ch_idx: (b[1] & 0x02) >> 1,
            cad_periodicity: (b[1] & 0x1c) >> 2,
            start_stop: (b[1] & 0x20) >> 5,
        })
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RelayConfReqPayload {
    pub channel_settings_relay: ChannelSettingsRelay,
    pub second_ch_freq: u32,
}

impl Payload for RelayConfReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 5];
        cur.read_exact(&mut b)?;

        Ok(RelayConfReqPayload {
            channel_settings_relay: ChannelSettingsRelay::from_slice(&b[3..5])?,
            second_ch_freq: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(&b[0..3]);
                let freq = u32::from_le_bytes(freq_b);

                if freq >= 12000000 {
                    // 2.4GHz frequency
                    freq * 200
                } else {
                    freq * 100
                }
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut freq = self.second_ch_freq;

        // Support LoRaWAN 2.4GHz, in which case the stepping is 200Hz:
        // See Frequency Encoding in MAC Commands
        // https://lora-developers.semtech.com/documentation/tech-papers-and-guides/physical-layer-proposal-2.4ghz/
        if freq >= 2400000000 {
            freq /= 2;
        }

        if freq / 100 >= (1 << 24) {
            return Err(anyhow!("max freq value is 2^24 - 1"));
        }
        if freq % 100 != 0 {
            return Err(anyhow!("freq must be multiple of 100"));
        }

        let mut b = vec![0; 5];
        b[0..3].copy_from_slice(&(freq / 100).to_le_bytes());
        b[3..5].copy_from_slice(&self.channel_settings_relay.to_bytes()?);
        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RelayConfAnsPayload {
    pub second_ch_freq_ack: bool,
    pub second_ch_ack_offset_ack: bool,
    pub second_ch_dr_ack: bool,
    pub second_ch_idx_ack: bool,
    pub default_ch_idx_ack: bool,
    pub cad_periodicity_ack: bool,
}

impl Payload for RelayConfAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(RelayConfAnsPayload {
            second_ch_freq_ack: b[0] & 0x01 != 0,
            second_ch_ack_offset_ack: b[0] & 0x02 != 0,
            second_ch_dr_ack: b[0] & 0x04 != 0,
            second_ch_idx_ack: b[0] & 0x08 != 0,
            default_ch_idx_ack: b[0] & 0x10 != 0,
            cad_periodicity_ack: b[0] & 0x20 != 0,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b: u8 = 0;

        if self.second_ch_freq_ack {
            b |= 0x01;
        }

        if self.second_ch_ack_offset_ack {
            b |= 0x02;
        }

        if self.second_ch_dr_ack {
            b |= 0x04;
        }

        if self.second_ch_idx_ack {
            b |= 0x08;
        }

        if self.default_ch_idx_ack {
            b |= 0x10;
        }

        if self.cad_periodicity_ack {
            b |= 0x20;
        }

        Ok(vec![b])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum RelayModeActivation {
    DisableRelayMode,
    EnableRelayMode,
    Dynamic,
    EndDeviceControlled,
}

impl RelayModeActivation {
    pub fn to_u8(&self) -> u8 {
        match self {
            RelayModeActivation::DisableRelayMode => 0x00,
            RelayModeActivation::EnableRelayMode => 0x01,
            RelayModeActivation::Dynamic => 0x02,
            RelayModeActivation::EndDeviceControlled => 0x03,
        }
    }

    pub fn from_u8(v: u8) -> Result<Self> {
        Ok(match v {
            0x00 => RelayModeActivation::DisableRelayMode,
            0x01 => RelayModeActivation::EnableRelayMode,
            0x02 => RelayModeActivation::Dynamic,
            0x03 => RelayModeActivation::EndDeviceControlled,
            _ => {
                return Err(anyhow!("invalid RelayModeActivation: {}", v));
            }
        })
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ActivationRelayMode {
    pub relay_mode_activation: RelayModeActivation,
    pub smart_enable_level: u8,
}

impl ActivationRelayMode {
    pub fn from_u8(b: u8) -> Result<Self> {
        Ok(ActivationRelayMode {
            relay_mode_activation: RelayModeActivation::from_u8((b & 0x0c) >> 2)?,
            smart_enable_level: b & 0x03,
        })
    }

    pub fn to_u8(&self) -> Result<u8> {
        if self.smart_enable_level > 3 {
            return Err(anyhow!("max value of smart_enable_level is 3"));
        }

        Ok((self.relay_mode_activation.to_u8() << 2) | self.smart_enable_level)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ChannelSettingsED {
    pub second_ch_ack_offset: u8,
    pub second_ch_dr: u8,
    pub second_ch_idx: u8,
    pub backoff: u8,
}

impl ChannelSettingsED {
    const SIZE: usize = 2;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("ChannelSettingsED expects 2 bytes"));
        }

        Ok(ChannelSettingsED {
            second_ch_ack_offset: b[0] & 0x07,
            second_ch_dr: (b[0] & 0x78) >> 3,
            second_ch_idx: (b[0] & 0x80) >> 7 | (b[1] & 0x01) << 1,
            backoff: (b[1] & 0x7e) >> 1,
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.second_ch_ack_offset > 7 {
            return Err(anyhow!("max value of second_ch_ack_offset is 7"));
        }
        if self.second_ch_dr > 15 {
            return Err(anyhow!("max value of second_ch_dr is 15"));
        }
        if self.second_ch_idx > 3 {
            return Err(anyhow!("max value of second_ch_idx is 3"));
        }
        if self.backoff > 63 {
            return Err(anyhow!("max value of backoff is 63"));
        }

        Ok([
            self.second_ch_ack_offset | (self.second_ch_dr << 3) | (self.second_ch_idx << 7),
            (self.second_ch_idx >> 1) | (self.backoff << 1),
        ])
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct EndDeviceConfReqPayload {
    pub activation_relay_mode: ActivationRelayMode,
    pub channel_settings_ed: ChannelSettingsED,
    pub second_ch_freq: u32,
}

impl Payload for EndDeviceConfReqPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 6];
        cur.read_exact(&mut b)?;

        Ok(EndDeviceConfReqPayload {
            second_ch_freq: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(&b[0..3]);
                let freq = u32::from_le_bytes(freq_b);

                if freq >= 12000000 {
                    // 2.4GHz frequency
                    freq * 200
                } else {
                    freq * 100
                }
            },
            channel_settings_ed: ChannelSettingsED::from_slice(&b[3..5])?,
            activation_relay_mode: ActivationRelayMode::from_u8(b[5])?,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut freq = self.second_ch_freq;

        // Support LoRaWAN 2.4GHz, in which case the stepping is 200Hz:
        // See Frequency Encoding in MAC Commands
        // https://lora-developers.semtech.com/documentation/tech-papers-and-guides/physical-layer-proposal-2.4ghz/
        if freq >= 2400000000 {
            freq /= 2;
        }

        if freq / 100 >= (1 << 24) {
            return Err(anyhow!("max freq value is 2^24 - 1"));
        }
        if freq % 100 != 0 {
            return Err(anyhow!("freq must be multiple of 100"));
        }

        let mut b = vec![0; 6];
        b[0..3].copy_from_slice(&(freq / 100).to_le_bytes());
        b[3..5].copy_from_slice(&self.channel_settings_ed.to_bytes()?);
        b[5] = self.activation_relay_mode.to_u8()?;
        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct EndDeviceConfAnsPayload {
    pub second_ch_freq_ack: bool,
    pub second_ch_dr_ack: bool,
    pub second_ch_idx_ack: bool,
    pub backoff_ack: bool,
}

impl Payload for EndDeviceConfAnsPayload {
    fn decode(cur: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut b = [0; 1];
        cur.read_exact(&mut b)?;

        Ok(EndDeviceConfAnsPayload {
            second_ch_freq_ack: b[0] & 0x01 != 0,
            second_ch_dr_ack: b[0] & 0x02 != 0,
            second_ch_idx_ack: b[0] & 0x04 != 0,
            backoff_ack: b[0] & 0x08 != 0,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b: u8 = 0;

        if self.second_ch_freq_ack {
            b |= 0x01;
        }
        if self.second_ch_dr_ack {
            b |= 0x02;
        }
        if self.second_ch_idx_ack {
            b |= 0x04;
        }
        if self.backoff_ack {
            b |= 0x08;
        }

        Ok(vec![b])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MACTest {
        uplink: bool,
        maccommand_set: MACCommandSet,
        bytes: Vec<u8>,
    }

    #[test]
    fn test_maccommand_set() {
        let tests = vec![
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::ResetInd(ResetIndPayload {
                    dev_lorawan_version: Version::LoRaWAN1_1,
                })]),
                bytes: vec![0x01, 0x01],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::ResetConf(ResetConfPayload {
                    serv_lorawan_version: Version::LoRaWAN1_1,
                })]),
                bytes: vec![0x01, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkCheckReq]),
                bytes: vec![0x02],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkCheckAns(
                    LinkCheckAnsPayload {
                        margin: 10,
                        gw_cnt: 15,
                    },
                )]),
                bytes: vec![0x02, 0x0a, 0x0f],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkADRReq(
                    LinkADRReqPayload {
                        dr: 1,
                        tx_power: 2,
                        ch_mask: ChMask::new({
                            let mut mask: [bool; 16] = [false; 16];
                            mask[2] = true;
                            mask
                        }),
                        redundancy: Redundancy {
                            ch_mask_cntl: 4,
                            nb_rep: 5,
                        },
                    },
                )]),
                bytes: vec![0x03, 0x12, 0x04, 0x00, 0x45],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkADRAns(
                    LinkADRAnsPayload {
                        ch_mask_ack: true,
                        dr_ack: false,
                        tx_power_ack: false,
                    },
                )]),
                bytes: vec![0x03, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkADRAns(
                    LinkADRAnsPayload {
                        ch_mask_ack: false,
                        dr_ack: true,
                        tx_power_ack: false,
                    },
                )]),
                bytes: vec![0x03, 0x02],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkADRAns(
                    LinkADRAnsPayload {
                        ch_mask_ack: false,
                        dr_ack: false,
                        tx_power_ack: true,
                    },
                )]),
                bytes: vec![0x03, 0x04],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkADRAns(
                    LinkADRAnsPayload {
                        ch_mask_ack: true,
                        dr_ack: true,
                        tx_power_ack: true,
                    },
                )]),
                bytes: vec![0x03, 0x07],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DutyCycleReq(
                    DutyCycleReqPayload { max_duty_cycle: 13 },
                )]),
                bytes: vec![0x04, 0x0d],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DutyCycleAns]),
                bytes: vec![0x04],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RxParamSetupReq(
                    RxParamSetupReqPayload {
                        frequency: 26265700,
                        dl_settings: DLSettings {
                            rx2_dr: 11,
                            rx1_dr_offset: 3,
                            opt_neg: false,
                        },
                    },
                )]),
                bytes: vec![0x05, 0x3b, 0x01, 0x02, 0x04],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RxParamSetupAns(
                    RxParamSetupAnsPayload {
                        channel_ack: true,
                        rx2_dr_ack: false,
                        rx1_dr_offset_ack: true,
                    },
                )]),
                bytes: vec![0x05, 0x05],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DevStatusReq]),
                bytes: vec![0x06],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DevStatusAns(
                    DevStatusAnsPayload {
                        battery: 0,
                        margin: -30,
                    },
                )]),
                bytes: vec![0x06, 0x00, 0x22],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DevStatusAns(
                    DevStatusAnsPayload {
                        battery: 255,
                        margin: 30,
                    },
                )]),
                bytes: vec![0x06, 0xff, 0x1e],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DevStatusAns(
                    DevStatusAnsPayload {
                        battery: 127,
                        margin: -1,
                    },
                )]),
                bytes: vec![0x06, 0x7f, 0x3f],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DevStatusAns(
                    DevStatusAnsPayload {
                        battery: 127,
                        margin: 0,
                    },
                )]),
                bytes: vec![0x06, 0x7f, 0x00],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelReq(
                    NewChannelReqPayload {
                        ch_index: 3,
                        freq: 26265700,
                        max_dr: 5,
                        min_dr: 10,
                    },
                )]),
                bytes: vec![0x07, 0x03, 0x01, 0x02, 0x04, 0x5a],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelReq(
                    NewChannelReqPayload {
                        ch_index: 3,
                        freq: 2410_000_000,
                        max_dr: 5,
                        min_dr: 0,
                    },
                )]),
                bytes: vec![7, 3, 80, 222, 183, 80],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelAns(
                    NewChannelAnsPayload {
                        channel_freq_ok: false,
                        dr_range_ok: false,
                    },
                )]),
                bytes: vec![0x07, 0x00],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelAns(
                    NewChannelAnsPayload {
                        channel_freq_ok: true,
                        dr_range_ok: false,
                    },
                )]),
                bytes: vec![0x07, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelAns(
                    NewChannelAnsPayload {
                        channel_freq_ok: false,
                        dr_range_ok: true,
                    },
                )]),
                bytes: vec![0x07, 0x02],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelAns(
                    NewChannelAnsPayload {
                        channel_freq_ok: true,
                        dr_range_ok: true,
                    },
                )]),
                bytes: vec![0x07, 0x03],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RxTimingSetupReq(
                    RxTimingSetupReqPayload { delay: 15 },
                )]),
                bytes: vec![0x08, 0x0f],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RxTimingSetupAns]),
                bytes: vec![0x08],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::TxParamSetupReq(
                    TxParamSetupReqPayload {
                        uplink_dwell_time: DwellTime::NoLimit,
                        downlink_dwell_time: DwellTime::NoLimit,
                        max_eirp: 15,
                    },
                )]),
                bytes: vec![0x09, 0x0f],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::TxParamSetupReq(
                    TxParamSetupReqPayload {
                        uplink_dwell_time: DwellTime::Limit400ms,
                        downlink_dwell_time: DwellTime::NoLimit,
                        max_eirp: 15,
                    },
                )]),
                bytes: vec![0x09, 0x1f],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::TxParamSetupReq(
                    TxParamSetupReqPayload {
                        uplink_dwell_time: DwellTime::NoLimit,
                        downlink_dwell_time: DwellTime::Limit400ms,
                        max_eirp: 15,
                    },
                )]),
                bytes: vec![0x09, 0x2f],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::TxParamSetupAns]),
                bytes: vec![0x09],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelReq(
                    DlChannelReqPayload {
                        ch_index: 0,
                        freq: 868100000,
                    },
                )]),
                bytes: vec![0x0a, 0x00, 0x28, 0x76, 0x84],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelReq(
                    DlChannelReqPayload {
                        ch_index: 1,
                        freq: 868200000,
                    },
                )]),
                bytes: vec![0x0a, 0x01, 0x10, 0x7a, 0x84],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelAns(
                    DlChannelAnsPayload {
                        uplink_freq_exists: false,
                        channel_freq_ok: false,
                    },
                )]),
                bytes: vec![0x0a, 0x00],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelAns(
                    DlChannelAnsPayload {
                        uplink_freq_exists: false,
                        channel_freq_ok: true,
                    },
                )]),
                bytes: vec![0x0a, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelAns(
                    DlChannelAnsPayload {
                        uplink_freq_exists: true,
                        channel_freq_ok: false,
                    },
                )]),
                bytes: vec![0x0a, 0x02],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelAns(
                    DlChannelAnsPayload {
                        uplink_freq_exists: true,
                        channel_freq_ok: true,
                    },
                )]),
                bytes: vec![0x0a, 0x03],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RekeyConf(RekeyConfPayload {
                    serv_lorawan_version: Version::LoRaWAN1_1,
                })]),
                bytes: vec![0x0b, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RekeyInd(RekeyIndPayload {
                    dev_lorawan_version: Version::LoRaWAN1_1,
                })]),
                bytes: vec![0x0b, 0x01],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::ADRParamSetupReq(
                    ADRParamSetupReqPayload {
                        adr_param: ADRParam {
                            limit_exp: 10,
                            delay_exp: 15,
                        },
                    },
                )]),
                bytes: vec![0x0c, 0xaf],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::ADRParamSetupAns]),
                bytes: vec![0x0c],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceTimeReq]),
                bytes: vec![0x0d],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceTimeAns(
                    DeviceTimeAnsPayload {
                        time_since_gps_epoch: Duration::from_secs(1),
                    },
                )]),
                bytes: vec![0x0d, 0x01, 0x00, 0x00, 0x00, 0x00],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceTimeAns(
                    DeviceTimeAnsPayload {
                        time_since_gps_epoch: Duration::new(1, 2 * 3906250),
                    },
                )]),
                bytes: vec![0x0d, 0x01, 0x00, 0x00, 0x00, 0x02],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::ForceRejoinReq(
                    ForceRejoinReqPayload {
                        period: 3,
                        max_retries: 4,
                        rejoin_type: 2,
                        dr: 5,
                    },
                )]),
                bytes: vec![0x0e, 0x25, 0x1c],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RejoinParamSetupReq(
                    RejoinParamSetupReqPayload {
                        max_time_n: 14,
                        max_count_n: 15,
                    },
                )]),
                bytes: vec![0x0f, 0xef],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RejoinParamSetupAns(
                    RejoinParamSetupAnsPayload { time_ok: true },
                )]),
                bytes: vec![0x0f, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotInfoReq(
                    PingSlotInfoReqPayload { periodicity: 3 },
                )]),
                bytes: vec![0x10, 0x03],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotInfoAns]),
                bytes: vec![0x10],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotChannelReq(
                    PingSlotChannelReqPayload {
                        freq: 868100000,
                        dr: 5,
                    },
                )]),
                bytes: vec![0x11, 0x28, 0x76, 0x84, 0x05],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotChannelAns(
                    PingSlotChannelAnsPayload {
                        dr_ok: false,
                        channel_freq_ok: false,
                    },
                )]),
                bytes: vec![0x011, 0x00],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotChannelAns(
                    PingSlotChannelAnsPayload {
                        dr_ok: false,
                        channel_freq_ok: true,
                    },
                )]),
                bytes: vec![0x011, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotChannelAns(
                    PingSlotChannelAnsPayload {
                        dr_ok: true,
                        channel_freq_ok: false,
                    },
                )]),
                bytes: vec![0x011, 0x02],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotChannelAns(
                    PingSlotChannelAnsPayload {
                        dr_ok: true,
                        channel_freq_ok: true,
                    },
                )]),
                bytes: vec![0x11, 0x03],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::BeaconFreqReq(
                    BeaconFreqReqPayload { freq: 868100000 },
                )]),
                bytes: vec![0x13, 0x28, 0x76, 0x84],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::BeaconFreqAns(
                    BeaconFreqAnsPayload {
                        beacon_freq_ok: false,
                    },
                )]),
                bytes: vec![0x13, 0x00],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::BeaconFreqAns(
                    BeaconFreqAnsPayload {
                        beacon_freq_ok: true,
                    },
                )]),
                bytes: vec![0x13, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceModeInd(
                    DeviceModeIndPayload {
                        class: DeviceModeClass::ClassA,
                    },
                )]),
                bytes: vec![0x20, 0x00],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceModeInd(
                    DeviceModeIndPayload {
                        class: DeviceModeClass::ClassC,
                    },
                )]),
                bytes: vec![0x20, 0x02],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceModeConf(
                    DeviceModeConfPayload {
                        class: DeviceModeClass::ClassA,
                    },
                )]),
                bytes: vec![0x20, 0x00],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceModeConf(
                    DeviceModeConfPayload {
                        class: DeviceModeClass::ClassC,
                    },
                )]),
                bytes: vec![0x20, 0x02],
            },
        ];

        for tst in tests {
            assert_eq!(tst.bytes, tst.maccommand_set.to_vec().unwrap());

            let mut maccommand_set = MACCommandSet::from_slice(&tst.bytes);
            maccommand_set.decode_from_raw(tst.uplink).unwrap();

            assert_eq!(tst.maccommand_set, maccommand_set);
        }
    }
}
