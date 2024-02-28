import { Pipe, PipeTransform } from "@angular/core";
import { TranslationService } from "./TranslationService";

@Pipe(
    {
        name: "translate",
        standalone: true,
    }
)
export class TranslatePipe implements PipeTransform {
    private canOperate: boolean = false;

    constructor(private translationService: TranslationService){
        this.translationService.dictionaryIsLoaded.subscribe(isLoaded => {
            this.canOperate = isLoaded;
        })
    };

    transform(value: any, ...args: any[]) {
        if (this.canOperate) {
            return this.translationService.translate(value);
        } else {
            return "Loading";
        }
    }
}