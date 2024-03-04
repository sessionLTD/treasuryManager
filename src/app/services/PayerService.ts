import { Injectable } from "@angular/core";
import { Payer } from "../models/payer";
import { invoke } from "@tauri-apps/api";
import { PayerCreationRequest } from "../models/requests/PayerCreationRequest";

@Injectable()
export class PayerSerivice {
    getAllPayers(): Promise<Payer[]> {
        return invoke<Payer[]>("get_all_payers");
    }

    saveNewPayer(payer: PayerCreationRequest): Promise<Payer> {
        return invoke<Payer>("create_new_payer", {payer: payer});
    }
}