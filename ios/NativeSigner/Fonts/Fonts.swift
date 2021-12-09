//
//  Fonts.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import Foundation
import SwiftUI

/**
 * Fonts definitions
 *
 * Use this place only to define fonts!
 */

//Fontstyles
enum Fontstyle {
    case h1
    case h2
    case h3
    case h4
    case button
    case body1
    case body2
    case overline
    case subtitle1
    case subtitle2
}

//Base
func FBase(style: Fontstyle) -> Font {
    switch style {
    case .h1:
        return Font.custom("Inter-Bold", size:19)
    case .h2:
        return Font.custom("Inter-Semibold", size:19)
    case .h3:
        return Font.custom("Inter-Semibold", size:16)
    case .h4:
        return Font.custom("Inter-Medium", size:16)
    case .button:
        return Font.custom("Inter-Semibold", size:16)
    case .body1:
        return Font.custom("Inter-Regular", size:16)
    case .body2:
        return Font.custom("Inter-Regular", size:15)
    case .overline:
        return Font.custom("Inter-Medium", size:12)
    case .subtitle1:
        return Font.custom("Inter-Medium", size:14)
    case .subtitle2:
        return Font.custom("Inter-Regular", size:13)
    }
}

//Crypto
func FCrypto(style: Fontstyle) -> Font {
    switch style {
    case .h1:
        return Font.custom("robotomono_regular", size:19).weight(.bold)
    case .h2:
        return Font.custom("robotomono_regular", size:19).weight(.semibold)
    case .h3:
        return Font.custom("robotomono_regular", size:16).weight(.semibold)
    case .h4:
        return Font.custom("robotomono_regular", size:16).weight(.medium)
    case .button:
        return Font.custom("robotomono_regular", size:16).weight(.semibold)
    case .body1:
        return Font.custom("robotomono_regular", size:12).weight(.medium)
    case .body2:
        return Font.custom("robotomono_regular", size:13).weight(.light)
    case .overline:
        return Font.custom("robotomono_regular", size:12).weight(.medium)
    case .subtitle1:
        return Font.custom("robotomono_regular", size:14).weight(.medium)
    case .subtitle2:
        return Font.custom("robotomono_regular", size:13).weight(.regular)
    }
}

//Web3
func FWeb3(style: Fontstyle) -> Font {
    switch style {
    case .h1:
        return Font.custom("Web3-Regular", size:19)
    case .h2:
        return Font.custom("Web3-Regular", size:19)
    case .h3:
        return Font.custom("Web3-Regular", size:16)
    case .h4:
        return Font.custom("Web3-Regular", size:16)
    case .button:
        return Font.custom("Web3-Regular", size:16)
    case .body1:
        return Font.custom("Web3-Regular", size:16)
    case .body2:
        return Font.custom("Web3-Regular", size:15)
    case .overline:
        return Font.custom("Web3-Regular", size:12)
    case .subtitle1:
        return Font.custom("Web3-Regular", size:14)
    case .subtitle2:
        return Font.custom("Web3-Regular", size:13)
    }
}
