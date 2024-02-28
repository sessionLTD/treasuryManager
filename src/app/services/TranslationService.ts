import { Injectable } from "@angular/core";
import { invoke } from "@tauri-apps/api";
import { BehaviorSubject, Subject } from "rxjs";

@Injectable()
export class TranslationService {
    public language: Subject<LanguageIdentifier | undefined> = new Subject();
    public dictionaryIsLoaded: BehaviorSubject<boolean> = new BehaviorSubject(false);
    private values?: {[key: string]: {[key: string]: string}};
    public languages = [
        LanguageIdentifier.English,
        LanguageIdentifier.German, 
        LanguageIdentifier.Norwegian
    ];

    constructor() {
        this.setup();
    }
    
    translate(key: string): string {
        if (this.values != undefined) {
            const split = key.split(".");
            let group = split[0];
            let object = split[1];
            let translation = this.values[group][object];
            if (translation != undefined) {
                return translation;
            } else {
                return "UNKNOWN";
            }
        } else {
            return "Loading"
        }
    }

    private getDictionary(langId: LanguageIdentifier) {
        this.dictionaryIsLoaded.next(false);
        invoke<string>("get_dictionary", {language: langId}).then(result => {
            this.values = JSON.parse(result);
            this.dictionaryIsLoaded.next(true);
        })
    }

    setLanguage(language: LanguageIdentifier) {
        invoke("set_language", {lanuage: language}).then(() => this.setup());
    }   
    
    
    private setup() {
        invoke<LanguageIdentifier>("get_language")
        .then(language => {
            this.language.next(language);
            this.getDictionary(language);
        })
    }
}

export enum LanguageIdentifier {
    German = "de-DE",
    English = "en",
    Norwegian = "no-BK"
}