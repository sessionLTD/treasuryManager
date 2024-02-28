import { Injectable } from "@angular/core";
import { Payer } from "../models/payer";
import { invoke } from "@tauri-apps/api";

@Injectable()
export class PayerSerivice {
    getAllPayers(): Promise<Payer[]> {
        return invoke<Payer[]>("get_all_payers");
    }
}