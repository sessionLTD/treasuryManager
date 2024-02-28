import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router, RouterOutlet } from '@angular/router';
import { invoke } from "@tauri-apps/api/tauri";
import { MainMenuComponent } from './components/main-menu/main-menu.component';
import { PayerDisplayComponent } from './components/main-menu/payer-display/payer-display.component';
import { TranslatePipe } from './services/TranslationPipe';
import { TranslationService } from './services/TranslationService';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    CommonModule,
    RouterOutlet, 
    MainMenuComponent
  ],
  providers: [TranslatePipe],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {

  public isDone: boolean = false;

  constructor(private translationSerivice: TranslationService) {
    this.translationSerivice.dictionaryIsLoaded.subscribe(isLoaded => {
      this.isDone = isLoaded;
    })
  }
    
}
