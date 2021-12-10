//
//  ScreenSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 26.11.2021.
//

import SwiftUI

struct ScreenSelector: View {
    @EnvironmentObject var data: SignerDataModel
    
    var body: some View {
        switch (data.actionResult.screen) {
        case .Scan :
            TransactionScreen()
        case .Keys(let value):
            KeyManager(content: value)
        case .Settings :
            SettingsScreen()
        case .Log(let value) :
            HistoryScreen(content: value)
        case .LogDetails:
            EventDetails()
        case .Transaction:
            TransactionPreview()
        case .SeedSelector(let value):
            SeedManager(content: value)
        case .KeyDetails(let value):
            ExportAddress(content: value)
        case .Backup:
            SeedBackup()
        case .NewSeed:
            NewSeedScreen()
        case .RecoverSeedName:
            RecoverSeedName()
        case .RecoverSeedPhrase:
            RecoverSeedPhrase()
        case .DeriveKey:
            NewAddressScreen()
        case .Verifier:
            Text("Verifier")
        case .ManageNetwork:
            Text("details of network")
            //NetworkDetails()
        }
    }
}

/*
struct ScreenSelector_Previews: PreviewProvider {
    static var previews: some View {
        ScreenSelector()
    }
}
*/
