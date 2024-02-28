import { ApplicationConfig } from "@angular/core";
import { provideRouter } from "@angular/router";
import { routes } from "./app.routes";
import { PayerSerivice } from "./services/PayerService";
import { TranslationService } from "./services/TranslationService";

export const appConfig: ApplicationConfig = {
  providers: [provideRouter(routes), PayerSerivice, TranslationService],
};
