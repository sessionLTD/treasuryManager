import { ApplicationConfig } from "@angular/core";
import { provideRouter } from "@angular/router";
import { routes } from "./app.routes";
import { PayerSerivice } from "./services/PayerService";
import { TranslationService } from "./services/TranslationService";
import { provideAnimations } from '@angular/platform-browser/animations';


export const appConfig: ApplicationConfig = {
  providers: [provideRouter(routes), PayerSerivice, TranslationService, provideAnimations()],
};
