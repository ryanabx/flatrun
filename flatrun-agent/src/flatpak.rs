use std::{io, path::PathBuf, thread::sleep, time::Duration};

use libflatpak::{
    gio::prelude::FileExt,
    glib::{Bytes, KeyFile, KeyFileFlags},
    prelude::{
        BundleRefExt, InstallationExt, InstallationExtManual, InstanceExt, RefExt, RemoteExt,
        TransactionExt,
    },
    BundleRef, Installation, LaunchFlags, Remote,
};

use crate::FlatrunAgentError;

const DATA: [u8; 2844] = [
    // Offset 0x00000000 to 0x00000B1B
    0x99, 0x02, 0x0D, 0x04, 0x59, 0x43, 0xDA, 0xC0, 0x01, 0x10, 0x00, 0xEC, 0x89, 0x46, 0x54, 0x39,
    0x80, 0x60, 0xD5, 0x47, 0x43, 0x69, 0x69, 0x04, 0x74, 0x96, 0x24, 0x4D, 0x26, 0x43, 0xEB, 0xCD,
    0xB5, 0xE2, 0x6F, 0x10, 0xD5, 0xF6, 0xEB, 0x3F, 0x90, 0x29, 0xB5, 0xA5, 0x1F, 0x0A, 0x5F, 0x0A,
    0x60, 0xA1, 0x4C, 0x36, 0x80, 0x09, 0x44, 0x15, 0xE7, 0xF4, 0x56, 0xC4, 0xEA, 0xAE, 0x95, 0x07,
    0x38, 0x21, 0x1D, 0x78, 0xFF, 0xAD, 0x29, 0xC0, 0xF1, 0x63, 0xE7, 0x91, 0xB6, 0x84, 0x59, 0x1E,
    0xF1, 0x96, 0xD3, 0xAA, 0xCC, 0x1D, 0x72, 0x1E, 0x90, 0x90, 0xD6, 0x5F, 0x31, 0x49, 0xF1, 0xF9,
    0x37, 0x4D, 0x83, 0x31, 0x40, 0x52, 0x46, 0xD0, 0xFF, 0xAC, 0xE6, 0x2C, 0x04, 0xA6, 0xDA, 0x25,
    0xC3, 0xA0, 0x5B, 0x42, 0xB6, 0xD6, 0xC5, 0xFE, 0x05, 0xE7, 0x0C, 0x63, 0xF0, 0x08, 0xDB, 0x20,
    0x03, 0xBC, 0x22, 0x44, 0x8E, 0x15, 0x4A, 0xAE, 0x34, 0x7D, 0xC9, 0xF4, 0x57, 0xF8, 0xD4, 0x04,
    0x84, 0x37, 0x1B, 0x5D, 0x41, 0x51, 0x1A, 0xDF, 0x59, 0x4F, 0xE4, 0xC8, 0xE7, 0x24, 0xF8, 0xC5,
    0xCC, 0x1B, 0xBE, 0xF4, 0xEE, 0xED, 0xA1, 0x54, 0x47, 0xF7, 0x47, 0xEF, 0xEC, 0x23, 0xAD, 0x0E,
    0xE5, 0xCE, 0x81, 0xA5, 0x64, 0x61, 0x45, 0xAD, 0x05, 0xD2, 0xB3, 0x65, 0x88, 0xD1, 0x66, 0x98,
    0x1D, 0x60, 0xDE, 0x0B, 0x0C, 0xA6, 0xB6, 0x88, 0x57, 0xAF, 0x21, 0x7C, 0x95, 0xC5, 0x90, 0x92,
    0x11, 0xFC, 0x52, 0x17, 0xCD, 0x25, 0x50, 0x49, 0x16, 0x00, 0x98, 0xF1, 0x0E, 0x94, 0x1E, 0x46,
    0x46, 0xD0, 0x5C, 0x4C, 0xBB, 0xCC, 0x67, 0x20, 0x30, 0x4C, 0x8C, 0x19, 0x92, 0x64, 0xAC, 0x0A,
    0x42, 0x7D, 0x27, 0xC3, 0x3C, 0x0C, 0xD9, 0xA7, 0xEC, 0x91, 0x91, 0x27, 0x98, 0xD3, 0xD8, 0x0C,
    0x8F, 0x27, 0xB8, 0xDB, 0x12, 0x64, 0x16, 0x52, 0x03, 0x0F, 0x93, 0x86, 0x19, 0x1D, 0x09, 0x91,
    0xD9, 0x65, 0x1C, 0x33, 0x9A, 0x3C, 0xF2, 0x46, 0xE2, 0xCB, 0x54, 0x5A, 0x4E, 0x8F, 0x75, 0xEB,
    0xC9, 0x7C, 0x19, 0x71, 0x27, 0x9F, 0x34, 0xC3, 0xC3, 0x91, 0x90, 0x2E, 0x59, 0x6C, 0xCF, 0x62,
    0x3C, 0x52, 0xF3, 0x47, 0x26, 0xDA, 0x11, 0xB5, 0x59, 0xFA, 0xAB, 0x1B, 0x41, 0xA5, 0xF3, 0xE8,
    0x97, 0xEB, 0xD7, 0x1B, 0x41, 0xC8, 0x5B, 0xEE, 0x06, 0xC4, 0x6F, 0x44, 0x6D, 0xA8, 0xDB, 0x24,
    0x4E, 0x8F, 0x43, 0x0E, 0x63, 0xB4, 0x6C, 0xD2, 0x8D, 0x9C, 0xC6, 0xF3, 0x98, 0x82, 0x77, 0xFE,
    0xB4, 0xDB, 0x49, 0xA8, 0x4A, 0x4B, 0x0B, 0x5E, 0xC2, 0x49, 0x6A, 0x48, 0xFF, 0xF4, 0xD6, 0x8D,
    0x16, 0x29, 0x98, 0xB2, 0xFA, 0x7F, 0xD1, 0x02, 0x45, 0x0A, 0x38, 0x50, 0x69, 0xAE, 0x32, 0x6D,
    0x58, 0xE6, 0x64, 0x73, 0xC4, 0x8F, 0x5D, 0x5F, 0xEB, 0x9E, 0x84, 0xB6, 0x75, 0x6B, 0x09, 0x59,
    0x06, 0x61, 0xB8, 0xF3, 0xED, 0xBF, 0x7B, 0x5E, 0x7D, 0xD0, 0xE2, 0x13, 0x2A, 0x7B, 0x78, 0xFA,
    0xDF, 0x81, 0xF2, 0xB1, 0xB8, 0x7A, 0x59, 0x11, 0x2D, 0x08, 0x65, 0x18, 0x85, 0x23, 0x82, 0x7F,
    0x9C, 0x05, 0x0F, 0xE5, 0xB8, 0xB1, 0xFA, 0xCA, 0xDC, 0x23, 0x13, 0x95, 0xA8, 0x06, 0xA6, 0xEB,
    0xC0, 0x9B, 0x02, 0x53, 0xEF, 0x59, 0x38, 0xAF, 0x45, 0x43, 0x11, 0x55, 0xB2, 0xA3, 0x26, 0x21,
    0x97, 0x20, 0xE2, 0x7B, 0xF3, 0x00, 0x0C, 0x8E, 0xA3, 0xE0, 0xDD, 0x02, 0x02, 0xD0, 0x64, 0x34,
    0x10, 0xF0, 0x16, 0xC0, 0x03, 0xB3, 0xCE, 0x97, 0x60, 0x6E, 0x71, 0xBB, 0x4C, 0x24, 0x1E, 0xB5,
    0xA5, 0x55, 0xF2, 0xB4, 0x98, 0x0A, 0xE8, 0xAB, 0x94, 0xF6, 0x0D, 0xF9, 0xB2, 0x48, 0xC0, 0x1B,
    0x64, 0x38, 0x2F, 0x52, 0x58, 0xBA, 0x0F, 0x31, 0x56, 0x53, 0xC1, 0x00, 0x11, 0x01, 0x00, 0x01,
    0xB4, 0x2E, 0x46, 0x6C, 0x61, 0x74, 0x68, 0x75, 0x62, 0x20, 0x52, 0x65, 0x70, 0x6F, 0x20, 0x53,
    0x69, 0x67, 0x6E, 0x69, 0x6E, 0x67, 0x20, 0x4B, 0x65, 0x79, 0x20, 0x3C, 0x66, 0x6C, 0x61, 0x74,
    0x68, 0x75, 0x62, 0x40, 0x66, 0x6C, 0x61, 0x74, 0x68, 0x75, 0x62, 0x2E, 0x6F, 0x72, 0x67, 0x3E,
    0x89, 0x02, 0x54, 0x04, 0x13, 0x01, 0x08, 0x00, 0x3E, 0x16, 0x21, 0x04, 0x6E, 0x5C, 0x05, 0xD9,
    0x79, 0xC7, 0x6D, 0xAF, 0x93, 0xC0, 0x81, 0x35, 0x41, 0x84, 0xDD, 0x4D, 0x90, 0x7A, 0x7C, 0xAE,
    0x05, 0x02, 0x59, 0x43, 0xDA, 0xC0, 0x02, 0x1B, 0x03, 0x05, 0x09, 0x12, 0xCC, 0x03, 0x00, 0x05,
    0x0B, 0x09, 0x08, 0x07, 0x02, 0x06, 0x15, 0x08, 0x09, 0x0A, 0x0B, 0x02, 0x04, 0x16, 0x02, 0x03,
    0x01, 0x02, 0x1E, 0x01, 0x02, 0x17, 0x80, 0x00, 0x0A, 0x09, 0x10, 0x41, 0x84, 0xDD, 0x4D, 0x90,
    0x7A, 0x7C, 0xAE, 0x51, 0x25, 0x0F, 0xFE, 0x3E, 0xD7, 0x78, 0xB1, 0x6C, 0x5A, 0x88, 0x05, 0xBD,
    0xD4, 0x51, 0x64, 0xEF, 0xEC, 0x26, 0x60, 0xE9, 0x04, 0x7B, 0x53, 0x58, 0xD9, 0x40, 0xCD, 0x26,
    0x31, 0x04, 0xCF, 0x7E, 0x0F, 0x34, 0xFF, 0xF5, 0x46, 0x8C, 0x6F, 0x78, 0x70, 0xED, 0xE3, 0x79,
    0x18, 0x25, 0x0D, 0xB7, 0x39, 0x66, 0x8F, 0x26, 0xE6, 0x40, 0x6E, 0xEF, 0x9F, 0x5A, 0xD8, 0xD1,
    0x61, 0xC3, 0x01, 0xCC, 0xEB, 0x0C, 0x09, 0xA1, 0x5C, 0x45, 0x21, 0xC5, 0x88, 0x32, 0x02, 0xF5,
    0xA4, 0xE9, 0xE2, 0xEC, 0x7F, 0x9A, 0x8F, 0x88, 0x11, 0xEC, 0x9A, 0xAD, 0x8B, 0x7F, 0xA3, 0x22,
    0x9B, 0xE6, 0xDE, 0x65, 0xB5, 0xAE, 0xB1, 0x68, 0x0A, 0xA0, 0xEE, 0xBC, 0x2D, 0xEB, 0x98, 0xD6,
    0xE7, 0xF6, 0x90, 0x8E, 0x74, 0xB8, 0x87, 0xB7, 0x85, 0x67, 0xE7, 0x40, 0x2A, 0xD6, 0xBB, 0x63,
    0xF7, 0xE5, 0x8C, 0xDA, 0xCE, 0xCE, 0x75, 0x29, 0xF3, 0x5F, 0xD2, 0x31, 0xCC, 0x78, 0x40, 0x0E,
    0x46, 0xD8, 0xFC, 0x34, 0x86, 0xCE, 0x17, 0xAF, 0x18, 0x39, 0xF6, 0x83, 0x8D, 0x39, 0x31, 0x46,
    0x06, 0x00, 0x3A, 0x14, 0x5F, 0x16, 0x42, 0x08, 0x6B, 0xA5, 0x1E, 0xD2, 0x9C, 0x47, 0xB4, 0x15,
    0x21, 0x0E, 0x56, 0xC0, 0x23, 0xC1, 0x0D, 0x77, 0x09, 0xF9, 0x1F, 0x6B, 0xB6, 0xFB, 0xD2, 0x89,
    0x97, 0x49, 0x6A, 0x7B, 0x19, 0x67, 0xD4, 0xCE, 0x00, 0x4A, 0xCA, 0x85, 0xB1, 0x29, 0x3A, 0xB3,
    0x30, 0xDD, 0xD1, 0x40, 0x78, 0xE3, 0x84, 0xF2, 0x5C, 0xB1, 0x09, 0xEB, 0xA8, 0x7E, 0x5E, 0x9D,
    0x4B, 0xB8, 0x3D, 0xC6, 0xF6, 0x2D, 0xC3, 0x05, 0xA7, 0x38, 0xA6, 0x1E, 0x75, 0x20, 0x6F, 0xD7,
    0xED, 0xEF, 0x60, 0xCF, 0xE7, 0x0C, 0x5F, 0xCC, 0x71, 0x80, 0x58, 0xE5, 0x22, 0x51, 0x71, 0xDB,
    0x01, 0xE9, 0x1F, 0xAB, 0x98, 0x85, 0x92, 0xCF, 0x1C, 0x85, 0xAF, 0xD1, 0x49, 0x3E, 0xB0, 0x93,
    0x11, 0xB7, 0x45, 0xFE, 0x38, 0x66, 0x7D, 0x3D, 0x68, 0xEC, 0x61, 0xD5, 0xBB, 0x4C, 0x70, 0x62,
    0xFE, 0xE7, 0x16, 0xF5, 0x1B, 0x40, 0x6E, 0x09, 0x09, 0xC0, 0xD1, 0xF3, 0xFF, 0x90, 0x36, 0x08,
    0x3F, 0x08, 0x7D, 0xCF, 0x6D, 0x84, 0xAB, 0x04, 0x50, 0x3A, 0xCA, 0x48, 0x72, 0xF1, 0x9E, 0x2D,
    0x59, 0x05, 0x78, 0x87, 0xEE, 0x6F, 0x1A, 0x5C, 0x31, 0xD3, 0x9F, 0x42, 0xA4, 0x27, 0x66, 0xD1,
    0xCC, 0x09, 0xA6, 0xD5, 0xE5, 0x0F, 0xD1, 0x93, 0xD2, 0xB5, 0x64, 0x67, 0x3C, 0x33, 0x68, 0x65,
    0x83, 0xC8, 0x07, 0x6D, 0x73, 0x05, 0x72, 0x1D, 0xB1, 0x27, 0x45, 0xAE, 0xEF, 0x67, 0x97, 0x6C,
    0x0F, 0xD4, 0x63, 0x93, 0x2C, 0x90, 0x47, 0x81, 0x81, 0xAC, 0x13, 0x42, 0x74, 0x0E, 0x2B, 0xFB,
    0xCD, 0x9B, 0xBB, 0x74, 0x00, 0x93, 0x29, 0x58, 0xAD, 0xBA, 0x42, 0x0E, 0xB5, 0xEF, 0xC5, 0x26,
    0x46, 0x08, 0x66, 0x79, 0x90, 0xE3, 0xF6, 0xE3, 0x51, 0x73, 0x71, 0x08, 0xD6, 0x74, 0x9B, 0x82,
    0xF9, 0x86, 0x96, 0x90, 0xE6, 0x0B, 0x16, 0x80, 0x1A, 0xA6, 0x86, 0x2F, 0x5C, 0xD6, 0xE0, 0xA3,
    0xD6, 0xE7, 0x23, 0x76, 0xE7, 0xFA, 0x9E, 0xA9, 0x77, 0x20, 0x0A, 0xF5, 0x23, 0x64, 0xB2, 0x3D,
    0x3C, 0xDA, 0xB7, 0x2A, 0xF8, 0xB3, 0xA7, 0x59, 0x54, 0xAD, 0xBC, 0x7B, 0x85, 0xE4, 0x62, 0x4A,
    0x86, 0xB3, 0x0A, 0xE7, 0x2F, 0x1D, 0x0D, 0x55, 0x1D, 0x4C, 0xDC, 0x3D, 0x52, 0xD4, 0x28, 0xA4,
    0xE4, 0xA1, 0x33, 0xFC, 0xA2, 0x44, 0xE1, 0x9D, 0x14, 0x51, 0xB0, 0x17, 0x19, 0xD7, 0xE8, 0xC8,
    0xE3, 0x23, 0x16, 0x20, 0x32, 0xCE, 0x09, 0x3E, 0xC8, 0x5B, 0x71, 0x5E, 0x3E, 0xBF, 0xB7, 0xC7,
    0x2B, 0xB2, 0xE5, 0xE1, 0x28, 0xF5, 0xAD, 0xB9, 0x02, 0x0D, 0x04, 0x59, 0x43, 0xDA, 0xEC, 0x01,
    0x10, 0x00, 0xB4, 0xFF, 0x0A, 0x64, 0xB2, 0x67, 0xC2, 0xD6, 0x95, 0x16, 0x10, 0x64, 0xA6, 0x30,
    0xE3, 0x5E, 0xE9, 0xED, 0x9D, 0xA1, 0xC6, 0xA7, 0x7A, 0x4E, 0x3C, 0x24, 0xC0, 0xCD, 0xC8, 0x43,
    0x58, 0xAB, 0xBA, 0x69, 0x16, 0x71, 0x36, 0x41, 0x64, 0x86, 0xC1, 0x82, 0xB4, 0x8E, 0x83, 0xA0,
    0x9C, 0x5C, 0x3D, 0xA8, 0x5B, 0x27, 0x42, 0x40, 0x75, 0xA2, 0xF6, 0xA6, 0xB1, 0x86, 0xE8, 0x44,
    0x97, 0x66, 0xC9, 0x2F, 0x92, 0xEC, 0x92, 0x60, 0x5C, 0x33, 0x15, 0x5D, 0x41, 0xE0, 0xF4, 0x2C,
    0xB1, 0xDD, 0xF1, 0x05, 0x27, 0x6E, 0x79, 0xC8, 0x6F, 0x2F, 0x28, 0x72, 0xF5, 0x8D, 0xD5, 0xA9,
    0x4F, 0x7E, 0xF2, 0x45, 0x8C, 0x25, 0x13, 0xAE, 0x60, 0xF6, 0xD9, 0x4D, 0x72, 0x70, 0xEC, 0xD1,
    0xC3, 0x69, 0x48, 0x7B, 0x89, 0x22, 0x40, 0xE3, 0x5B, 0x39, 0x9B, 0x5D, 0x5A, 0xB5, 0x48, 0x99,
    0x04, 0x12, 0x9D, 0xD8, 0xC5, 0x96, 0x2B, 0x22, 0xB5, 0xB9, 0xA5, 0x58, 0x7B, 0xC7, 0x9B, 0x69,
    0x4E, 0x39, 0x8C, 0xBC, 0xF8, 0x62, 0x41, 0xED, 0x87, 0xDF, 0x55, 0x5D, 0x7D, 0xDE, 0x19, 0xA7,
    0x05, 0x22, 0x5F, 0x7C, 0x2C, 0xB8, 0x6E, 0xF9, 0x2E, 0x2F, 0x1B, 0x08, 0xFA, 0x7D, 0x43, 0x84,
    0xEC, 0xDC, 0xEC, 0xC3, 0xD5, 0xDF, 0x3E, 0x87, 0x1A, 0x4E, 0x07, 0x88, 0x47, 0x05, 0xD3, 0x2C,
    0xAD, 0xF6, 0xD0, 0x98, 0x86, 0x9B, 0x31, 0x1C, 0x57, 0x41, 0xA2, 0xAD, 0xA1, 0xEF, 0xBD, 0x47,
    0x7C, 0x07, 0x12, 0xD2, 0xCF, 0x7C, 0x11, 0x50, 0x2A, 0x60, 0xC1, 0x67, 0x7F, 0xD7, 0xA6, 0xFB,
    0x87, 0x27, 0x62, 0x97, 0x48, 0xEA, 0x68, 0x48, 0x7D, 0x2D, 0x45, 0x42, 0x22, 0xA5, 0xE8, 0x30,
    0x40, 0x60, 0x0F, 0x2D, 0x4F, 0x78, 0x20, 0x95, 0xAD, 0x34, 0x0E, 0xEA, 0xCF, 0xA3, 0x42, 0x70,
    0x5F, 0xFC, 0xCD, 0xF8, 0xF6, 0x8E, 0xE6, 0x1F, 0xFA, 0x3B, 0xD6, 0x82, 0x67, 0x05, 0x7B, 0x1B,
    0xAA, 0xDB, 0x92, 0x03, 0x2E, 0xA8, 0xC7, 0xAB, 0x81, 0x76, 0x9F, 0xF2, 0x83, 0xF7, 0xF2, 0x0A,
    0xF4, 0xCF, 0xE3, 0x2A, 0x5A, 0x79, 0x1F, 0x33, 0xAD, 0xC1, 0xA1, 0xDA, 0xBA, 0xF0, 0x63, 0xF8,
    0x97, 0x88, 0x5D, 0xD3, 0x60, 0x68, 0x14, 0x16, 0xC3, 0x7F, 0x7F, 0x13, 0x32, 0xE1, 0x90, 0x07,
    0x5E, 0x3E, 0xB7, 0x02, 0x4C, 0x75, 0xF6, 0xC0, 0xC6, 0x8D, 0x8C, 0x6E, 0x09, 0x6E, 0xBD, 0x08,
    0x1F, 0xC0, 0x77, 0xD9, 0xD9, 0xC6, 0xBA, 0x83, 0x21, 0x18, 0xFB, 0xDB, 0x3F, 0x60, 0x3D, 0x60,
    0xBA, 0x02, 0xE3, 0x57, 0xE4, 0xBA, 0x08, 0x1E, 0x75, 0x5A, 0x41, 0x4D, 0x7B, 0x5C, 0xF8, 0xB4,
    0x03, 0xE0, 0x25, 0x93, 0x71, 0x51, 0xC2, 0x82, 0x6D, 0xE6, 0x87, 0x35, 0xBB, 0x61, 0x97, 0xC1,
    0xE4, 0xB3, 0xFA, 0x58, 0xF5, 0x10, 0x84, 0xB1, 0xE5, 0xFF, 0x11, 0xD7, 0xF3, 0x17, 0x5F, 0x1E,
    0xA5, 0xB4, 0xA3, 0x24, 0xC3, 0xE1, 0xE0, 0x33, 0xD0, 0x5D, 0xB5, 0x28, 0x17, 0x09, 0xB2, 0xB9,
    0x87, 0x28, 0x04, 0xE3, 0xE6, 0xBD, 0x91, 0xCD, 0x97, 0xD8, 0x4A, 0xEC, 0x1F, 0xF5, 0x44, 0xFF,
    0x30, 0xD5, 0x9A, 0xD7, 0x93, 0xCA, 0x9D, 0xBA, 0x90, 0xB4, 0x48, 0xD6, 0xDA, 0x7D, 0x7E, 0xF6,
    0xEF, 0x40, 0x7B, 0x94, 0xC3, 0x15, 0x22, 0x6C, 0xD9, 0x7A, 0xD1, 0xAC, 0x33, 0xDC, 0xA9, 0x5C,
    0x9E, 0xA5, 0x96, 0x1A, 0x77, 0x17, 0x05, 0xB4, 0xF6, 0x2B, 0x1E, 0x37, 0x21, 0x89, 0x18, 0xFB,
    0x2B, 0xFD, 0xDB, 0xD3, 0x30, 0x60, 0xD7, 0xA8, 0xE3, 0xDF, 0x53, 0xFF, 0x52, 0x01, 0x2D, 0x4B,
    0x49, 0x36, 0x38, 0x2B, 0xD3, 0xDB, 0x46, 0xD7, 0x6B, 0x04, 0x4D, 0xF5, 0xF1, 0x85, 0xFC, 0x98,
    0x89, 0x39, 0x00, 0x11, 0x01, 0x00, 0x01, 0x89, 0x04, 0x72, 0x04, 0x18, 0x01, 0x08, 0x00, 0x26,
    0x16, 0x21, 0x04, 0x6E, 0x5C, 0x05, 0xD9, 0x79, 0xC7, 0x6D, 0xAF, 0x93, 0xC0, 0x81, 0x35, 0x41,
    0x84, 0xDD, 0x4D, 0x90, 0x7A, 0x7C, 0xAE, 0x05, 0x02, 0x59, 0x43, 0xDA, 0xEC, 0x02, 0x1B, 0x02,
    0x05, 0x09, 0x12, 0xCC, 0x03, 0x00, 0x02, 0x40, 0x09, 0x10, 0x41, 0x84, 0xDD, 0x4D, 0x90, 0x7A,
    0x7C, 0xAE, 0xC1, 0x74, 0x20, 0x04, 0x19, 0x01, 0x08, 0x00, 0x1D, 0x16, 0x21, 0x04, 0x54, 0xA6,
    0xCD, 0xDD, 0x89, 0x19, 0xFB, 0x20, 0x42, 0x00, 0xD8, 0xAC, 0x56, 0x27, 0x02, 0xE9, 0xE3, 0xED,
    0x7E, 0xE8, 0x05, 0x02, 0x59, 0x43, 0xDA, 0xEC, 0x00, 0x0A, 0x09, 0x10, 0x56, 0x27, 0x02, 0xE9,
    0xE3, 0xED, 0x7E, 0xE8, 0x47, 0xA0, 0x0F, 0xFF, 0x42, 0x98, 0x9A, 0x20, 0x05, 0x7A, 0xAC, 0x75,
    0xE1, 0x9E, 0x37, 0xFF, 0xAB, 0x36, 0x82, 0xDD, 0xD5, 0x87, 0x19, 0x52, 0x77, 0xC6, 0xE6, 0x71,
    0x55, 0x7B, 0xA0, 0x91, 0x5B, 0x32, 0x17, 0x47, 0x04, 0x87, 0x96, 0x06, 0x9D, 0x81, 0xBE, 0xC5,
    0x1F, 0xD3, 0x42, 0x34, 0x66, 0x5D, 0x78, 0x66, 0x4C, 0x02, 0x29, 0xF2, 0xF0, 0x6D, 0xB3, 0x91,
    0x0B, 0x67, 0x88, 0xC6, 0xFC, 0xC9, 0xF9, 0x05, 0xA3, 0x19, 0xF3, 0xE1, 0x0E, 0x64, 0xB8, 0x5D,
    0x44, 0xA6, 0x86, 0x55, 0xF1, 0x5A, 0x76, 0x04, 0xBD, 0xE4, 0x9E, 0x93, 0x94, 0xDD, 0x7B, 0xE7,
    0xE6, 0xBB, 0xA9, 0x6A, 0x28, 0xCF, 0x02, 0x1E, 0x0E, 0x41, 0x37, 0x1E, 0x06, 0x58, 0x85, 0xFF,
    0xBD, 0xD2, 0xE2, 0xD7, 0x05, 0x89, 0x67, 0x28, 0x6B, 0x53, 0x0C, 0x08, 0x08, 0x0C, 0x64, 0xBA,
    0x12, 0xB1, 0xB8, 0xFD, 0x99, 0xD4, 0x09, 0xF8, 0xF3, 0xDD, 0xF4, 0x7A, 0xC9, 0x53, 0x0F, 0x61,
    0x86, 0x72, 0x30, 0x72, 0x73, 0x6E, 0xFE, 0xF5, 0xC2, 0x19, 0x8C, 0x72, 0x9A, 0xD3, 0xB2, 0x3F,
    0x68, 0xB7, 0xDF, 0xA3, 0x4E, 0x15, 0xFA, 0x83, 0x13, 0xF7, 0xFB, 0x60, 0xCE, 0x8A, 0xAE, 0xCE,
    0x0A, 0x55, 0x57, 0x88, 0xF0, 0x7A, 0x40, 0x03, 0xA3, 0x63, 0x09, 0x8E, 0x19, 0x24, 0x3E, 0xF7,
    0x92, 0x34, 0x8E, 0x36, 0xD1, 0xB8, 0x13, 0xF9, 0x0E, 0xC5, 0x8B, 0x59, 0x74, 0x5F, 0x56, 0x85,
    0x52, 0x78, 0x81, 0xBD, 0x03, 0x2C, 0x68, 0x16, 0xB6, 0xEC, 0xF9, 0xE0, 0x5A, 0x6E, 0xB1, 0x13,
    0x66, 0x58, 0xBE, 0x1E, 0xE7, 0x58, 0x27, 0x5D, 0xE6, 0x0D, 0x1C, 0xD1, 0xE2, 0xC8, 0x1F, 0x15,
    0xB0, 0xCD, 0x5D, 0x65, 0x67, 0x84, 0x72, 0xC7, 0x82, 0x13, 0x3B, 0xF5, 0x4B, 0xDB, 0x92, 0x2D,
    0x10, 0x27, 0x23, 0x85, 0x34, 0x5D, 0x61, 0xAE, 0xB8, 0xB0, 0x0E, 0xB6, 0x31, 0x84, 0x9A, 0x07,
    0x4C, 0x77, 0x79, 0x1F, 0x92, 0x8A, 0x1A, 0x60, 0x3F, 0x7C, 0x12, 0x34, 0xF1, 0x6E, 0xDC, 0xC2,
    0x68, 0xE8, 0x52, 0x93, 0x15, 0xD8, 0xB8, 0x08, 0x10, 0x04, 0x9A, 0x95, 0x87, 0x15, 0x6E, 0xC5,
    0x71, 0x19, 0x9D, 0xF9, 0xC4, 0x17, 0x78, 0x5A, 0xC7, 0x2C, 0xDD, 0xE2, 0x35, 0x55, 0xAB, 0xCB,
    0x2D, 0x07, 0x2B, 0xF0, 0x94, 0x10, 0x99, 0xA2, 0x64, 0x49, 0xA9, 0x36, 0x25, 0xDE, 0x72, 0x5E,
    0x2F, 0x3C, 0x82, 0x2E, 0xC8, 0x5D, 0x33, 0x81, 0x20, 0xB3, 0xB5, 0x0B, 0x4A, 0xA0, 0xCA, 0xFE,
    0x8A, 0x78, 0xE9, 0x99, 0x01, 0x82, 0xFE, 0xB1, 0x7F, 0x10, 0xB0, 0x55, 0x65, 0x35, 0x12, 0x2C,
    0xD0, 0x77, 0xE7, 0x44, 0xB4, 0x66, 0xDB, 0xF0, 0xD0, 0x52, 0x96, 0x0C, 0x02, 0x52, 0xC4, 0x59,
    0xA8, 0xC6, 0xDD, 0xF1, 0xC9, 0xCA, 0xAF, 0xBE, 0xDE, 0x3B, 0xAF, 0x14, 0xEB, 0x86, 0x2E, 0xFC,
    0x9E, 0x49, 0x3A, 0x8C, 0x04, 0xCA, 0x1B, 0x2B, 0xCF, 0xE7, 0x9B, 0x0E, 0xF8, 0xAE, 0xD0, 0xED,
    0x68, 0x74, 0x0D, 0x70, 0x3A, 0xF6, 0x1B, 0xCF, 0xAD, 0x85, 0xB6, 0x7A, 0xEF, 0x23, 0x18, 0x27,
    0x55, 0x1B, 0x18, 0xBA, 0x1D, 0x7E, 0xCB, 0xD6, 0xA7, 0x1F, 0x11, 0xDD, 0x37, 0xEF, 0xC4, 0xBB,
    0xD5, 0xD6, 0x77, 0x83, 0xDA, 0x3A, 0x28, 0xA9, 0x75, 0x3A, 0xC0, 0xD6, 0x59, 0x0E, 0x8C, 0x27,
    0xA1, 0x46, 0x05, 0x97, 0x9D, 0x97, 0xF8, 0x9B, 0x45, 0xFB, 0x3C, 0xA7, 0x9F, 0xAC, 0x6C, 0x25,
    0xF7, 0x95, 0x04, 0xA3, 0x10, 0x5C, 0xB7, 0xAF, 0x1E, 0xA7, 0xD2, 0xB5, 0x2B, 0x5F, 0x1F, 0x14,
    0xDA, 0xA6, 0xFE, 0x44, 0x22, 0xF3, 0x26, 0x59, 0x82, 0xE4, 0xC3, 0x41, 0x39, 0x30, 0xC9, 0xC0,
    0x73, 0xDC, 0x5F, 0xA8, 0x0A, 0xB1, 0x47, 0x17, 0x66, 0xB1, 0x10, 0x00, 0xB9, 0x91, 0x1E, 0x8A,
    0x37, 0x4A, 0xB7, 0xB0, 0x3B, 0x8D, 0x51, 0x0A, 0xBF, 0xB9, 0x9D, 0x49, 0x6A, 0x8A, 0xA5, 0x77,
    0x02, 0x88, 0x41, 0x32, 0x19, 0xD0, 0x6C, 0x6C, 0xF2, 0x1D, 0xFD, 0xBE, 0x76, 0xBB, 0xAD, 0x21,
    0xAA, 0x21, 0xED, 0x18, 0xFC, 0xD1, 0x74, 0x61, 0xD9, 0xB9, 0x24, 0xB8, 0xF2, 0xF0, 0x94, 0xD3,
    0xDF, 0xF9, 0xB4, 0xD7, 0xC0, 0xD5, 0x46, 0xB1, 0xEE, 0x1B, 0x7A, 0x3E, 0xC1, 0xDE, 0x85, 0x7A,
    0x01, 0xED, 0x4D, 0xEE, 0x54, 0xCD, 0x54, 0x5E, 0x10, 0x87, 0xED, 0x4E, 0xA3, 0x0B, 0x46, 0x8E,
    0x67, 0xC9, 0x3D, 0x6A, 0x34, 0xCF, 0x5B, 0x3A, 0x1C, 0x52, 0x55, 0xC6, 0x82, 0x28, 0x7F, 0xDC,
    0xF9, 0xF0, 0xCB, 0x2F, 0x81, 0xAA, 0x34, 0xCE, 0xB2, 0x86, 0xA9, 0xA3, 0x16, 0x68, 0xD4, 0x3A,
    0x2D, 0x47, 0x82, 0xF6, 0x0F, 0x40, 0x8F, 0xFF, 0x54, 0xAA, 0x36, 0xE8, 0xF1, 0x7E, 0x78, 0x68,
    0xB8, 0x98, 0xB3, 0xC6, 0x75, 0x58, 0xD7, 0x34, 0xEA, 0x32, 0x8F, 0xFA, 0x0B, 0xFC, 0x01, 0x62,
    0xED, 0xBF, 0xA0, 0x91, 0xFA, 0x5D, 0xA4, 0x2D, 0xEC, 0xF5, 0x60, 0xC0, 0xEE, 0x72, 0x39, 0xBB,
    0xBF, 0xA8, 0xAE, 0x08, 0xC5, 0x40, 0x99, 0x97, 0x66, 0xC6, 0x18, 0x39, 0x23, 0xF5, 0xF6, 0xF8,
    0xE3, 0x37, 0xBC, 0x1B, 0x2A, 0x3C, 0x37, 0x50, 0xA6, 0xBD, 0x93, 0x3C, 0x32, 0xD7, 0xDE, 0xB0,
    0xAB, 0x48, 0x18, 0x92, 0xD2, 0x03, 0x90, 0xE8, 0x3C, 0x0D, 0xE4, 0xDE, 0x50, 0x79, 0x39, 0x31,
    0x99, 0x8F, 0xFB, 0x40, 0xEF, 0x1B, 0x80, 0xBA, 0x3F, 0x5B, 0x3B, 0xB5, 0x8A, 0x29, 0x47, 0xE2,
    0xF3, 0x0A, 0x5B, 0xA2, 0xC1, 0x28, 0x26, 0x91, 0xC7, 0x95, 0x04, 0x27, 0x12, 0xA9, 0x2F, 0x8C,
    0x24, 0x80, 0x84, 0xEC, 0x60, 0xA9, 0xDE, 0x22, 0xDD, 0xCD, 0x4C, 0x7A, 0x39, 0x10, 0xEB, 0x9A,
    0x62, 0x4F, 0xB2, 0xDB, 0xE0, 0xEF, 0xB7, 0xA1, 0x10, 0x16, 0x2B, 0x8C, 0xB1, 0xD6, 0xDF, 0x52,
    0x3B, 0x37, 0xB1, 0xE2, 0x07, 0xB2, 0x08, 0x57, 0xFD, 0xC6, 0xCF, 0x25, 0xAE, 0x18, 0x52, 0xA6,
    0x40, 0x2F, 0xC0, 0xB8, 0x3A, 0xD5, 0x47, 0x6A, 0x28, 0xF0, 0x1A, 0x8D, 0x32, 0x9A, 0xDF, 0x4C,
    0x5A, 0xF4, 0xBD, 0xC8, 0xD2, 0x8C, 0xC9, 0x3D, 0x09, 0x84, 0xDD, 0xBF, 0x05, 0xD7, 0x8E, 0x55,
    0x6A, 0xC7, 0x93, 0xBC, 0x17, 0x5C, 0x99, 0xFF, 0x66, 0x3A, 0x57, 0x8A, 0x58, 0x9C, 0x5C, 0xA3,
    0xF1, 0xE2, 0xE7, 0x42, 0x17, 0xBE, 0xA6, 0x78, 0x4F, 0xBA, 0x37, 0x6B, 0xBF, 0x4F, 0xE4, 0x58,
    0x67, 0xB7, 0xCF, 0xD5, 0x03, 0x04, 0xB4, 0xD6, 0x4D, 0x99, 0x80, 0xCE, 0x7C, 0x0E, 0xAC, 0x56,
    0x2B, 0x8F, 0x0D, 0xF4, 0x12, 0x1C, 0xB3, 0x5B, 0x6A, 0xE2, 0x41, 0xA3, 0xF9, 0xF5, 0xB1, 0x16,
    0x00, 0xB0, 0xBE, 0x66, 0x70, 0x6D, 0xB6, 0x50, 0x37, 0x21, 0x26, 0xB9, 0x4A, 0xBE, 0x62, 0xB7,
    0x10, 0xE0, 0x03, 0x09, 0x08, 0x3E, 0x9B, 0x2A, 0xE6, 0xF3, 0x4D, 0x3A, 0x2E, 0x5B, 0x6E, 0x6C,
    0xD0, 0x78, 0xC1, 0x63, 0x73, 0x1C, 0x32, 0x74, 0x2E, 0xCD, 0x89, 0x1B, 0x34, 0x19, 0x09, 0xDF,
    0xF1, 0x1B, 0x25, 0x66, 0x52, 0x28, 0x9D, 0x08, 0x15, 0x2B, 0x0F, 0x9D, 0xFB, 0x59, 0xD9, 0x27,
    0x39, 0x81, 0x38, 0x14, 0x3E, 0xCF, 0xD1, 0xC1, 0xDA, 0x33, 0xD6, 0xEB, 0x28, 0x9F, 0xCC, 0xF0,
    0xB9, 0x04, 0x1F, 0x4C, 0xD4, 0x96, 0x8E, 0xF7, 0x6D, 0x8E, 0x17, 0x6A, 0x06, 0xE7, 0x0A, 0xAB,
    0xDE, 0xE8, 0xFB, 0xD0, 0x6F, 0x91, 0xB6, 0x67, 0x9E, 0x6A, 0x85, 0x6A, 0x61, 0x77, 0xA7, 0xF8,
    0x07, 0xA0, 0xC6, 0x79, 0xDF, 0x3A, 0xCC, 0x91, 0x1A, 0xFD, 0x18, 0x52,
];

pub fn install_bundle(
    installation: PathBuf,
    deps_installation: PathBuf,
    path: PathBuf,
) -> Result<(), FlatrunAgentError> {
    let bundle_install = get_repo(installation, true)?;
    let bundle_transaction = libflatpak::Transaction::for_installation(
        &bundle_install,
        libflatpak::gio::Cancellable::current().as_ref(),
    )?;
    // FIXME: https://github.com/ryanabx/flatrun/issues/6 (Upstream libflatpak issue)
    // let dep_install = get_repo(deps_installation, true)?;
    // TODO: Remove the below two lines and uncomment above line when issue is resolved.
    bundle_transaction.add_default_dependency_sources();
    let dep_install = system_repo()?;
    let dep_transaction = libflatpak::Transaction::for_installation(
        &dep_install,
        libflatpak::gio::Cancellable::current().as_ref(),
    )?;
    // Set up operations
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, path.to_string_lossy()).into());
    }
    let app_path_file = libflatpak::gio::File::for_path(&path);
    let bundle = BundleRef::new(&app_path_file)?;
    let metadata = KeyFile::new();
    metadata.load_from_bytes(&bundle.metadata().unwrap(), KeyFileFlags::empty())?;
    let app_name = bundle.name().unwrap().to_string();
    let branch = bundle.branch().unwrap();
    let runtime_name = metadata
        .string("Application", "runtime")
        .unwrap()
        .to_string();
    if let Err(e) =
        dep_transaction.add_install("flathub", &format!("runtime/{}", runtime_name), &[])
    {
        log::warn!("{e}");
    }
    log::debug!("{}", dep_install.path().unwrap().uri());
    // FIXME: See above fixme
    // bundle_transaction.add_sideload_repo(
    //     &dep_install
    //         .path()
    //         .unwrap()
    //         .path()
    //         .unwrap()
    //         .to_string_lossy(),
    // );
    // bundle_transaction.add_install("flathub", &format!("runtime/{}", runtime_name), &[])?;
    log::debug!("Runtime: {}", &format!("runtime/{}", runtime_name));
    bundle_transaction.add_install_bundle(&app_path_file, None)?;
    // Connect operations to print
    dep_transaction.connect_new_operation(move |_, transaction, progress| {
        let current_action = format!(
            "{}::{}",
            transaction.operation_type().to_str().unwrap(),
            transaction.get_ref().unwrap()
        );
        println!(
            "DEPS::{}::{}::{}",
            current_action,
            progress.status().unwrap_or_default().to_string(),
            progress.progress()
        );
        progress.connect_changed(move |progress| {
            println!(
                "DEPS::{}::{}::{}",
                current_action,
                progress.status().unwrap_or_default().to_string(),
                progress.progress()
            );
        });
    });
    bundle_transaction.connect_new_operation(move |_, transaction, progress| {
        let current_action = format!(
            "{}::{}",
            transaction
                .operation_type()
                .to_str()
                .unwrap()
                .to_uppercase(),
            transaction.get_ref().unwrap()
        );
        println!(
            "TEMP::{}::{}::{}",
            current_action,
            progress.status().unwrap_or_default().to_string(),
            progress.progress()
        );
        progress.connect_changed(move |progress| {
            println!(
                "TEMP::{}::{}::{}",
                current_action,
                progress.status().unwrap_or_default().to_string(),
                progress.progress()
            );
        });
    });
    // Run operations
    dep_transaction.run(libflatpak::gio::Cancellable::current().as_ref())?;
    log::debug!("Installing application {:?}", app_name);
    bundle_transaction.run(libflatpak::gio::Cancellable::current().as_ref())?;
    log::debug!("{}, {}", app_name, runtime_name);
    log::debug!(
        "temp_installation: {:?}",
        bundle_install
            .list_installed_refs(libflatpak::gio::Cancellable::current().as_ref())?
            .iter()
            .map(|x| { x.format_ref().unwrap() })
            .collect::<Vec<_>>()
    );

    // Run bundle
    let inst = bundle_install.launch_full(
        LaunchFlags::NONE,
        &app_name,
        None,
        Some(&branch),
        None,
        libflatpak::gio::Cancellable::current().as_ref(),
    );
    match inst {
        Ok(i) => {
            while i.is_running() {
                sleep(Duration::from_millis(1000));
            }
            log::info!("Instance is no longer running! Exiting...");
            Ok(())
        }
        Err(e) => {
            log::error!("{}", e);
            eprintln!("{:?}", e);
            Ok(())
        }
    }
}

fn system_repo() -> Result<Installation, FlatrunAgentError> {
    Ok(libflatpak::Installation::new_system(
        libflatpak::gio::Cancellable::current().as_ref(),
    )?)
}

fn get_repo(repo: PathBuf, add_flathub: bool) -> Result<Installation, FlatrunAgentError> {
    let repo_file = libflatpak::gio::File::for_path(repo);
    // Create installation
    let installation = libflatpak::Installation::for_path(
        &repo_file,
        true,
        libflatpak::gio::Cancellable::current().as_ref(),
    )?;
    if add_flathub {
        // Add flathub
        if let Err(e) = installation.add_remote(
            &flathub_remote(),
            false,
            libflatpak::gio::Cancellable::current().as_ref(),
        ) {
            log::warn!("{}", e);
            installation.modify_remote(
                &flathub_remote(),
                libflatpak::gio::Cancellable::current().as_ref(),
            )?;
        }
    }
    Ok(installation)
}

pub fn flathub_remote() -> Remote {
    let flathub = Remote::new("flathub");
    flathub.set_url("https://dl.flathub.org/repo/");
    flathub.set_homepage("https://flathub.org");
    flathub.set_comment("Central repository of Flatpak applications");
    flathub.set_description("Central repository of Flatpak applications");
    flathub.set_icon("https://dl.flathub.org/repo/logo.svg");
    // TODO: Get binary gpg key
    flathub.set_gpg_key(&Bytes::from(&DATA));
    flathub.set_gpg_verify(true);
    flathub.set_collection_id(Some("org.flathub.Stable"));
    flathub
}
