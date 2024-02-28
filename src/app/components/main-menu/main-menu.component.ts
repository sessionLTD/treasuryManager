import { ChangeDetectionStrategy, ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { PayerSerivice } from '../../services/PayerService';
import { Payer, PayerID } from '../../models/payer';
import { PayerDisplayComponent } from './payer-display/payer-display.component';
import { TranslatePipe } from '../../services/TranslationPipe';
import { TranslationService } from '../../services/TranslationService';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';

@Component({
  selector: 'main-menu',
  standalone: true,
  imports: [
    TranslatePipe,
    PayerDisplayComponent,
    CommonModule
  ],
  templateUrl: './main-menu.component.html',
  styleUrl: './main-menu.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class MainMenuComponent implements OnInit{
  payers: Payer[] = []

  constructor(
    private payerService: PayerSerivice,
    private translationService: TranslationService,
    private cd: ChangeDetectorRef  
  ) {
    this.payerService.getAllPayers().then(payers => {
      this.payers = payers;
    });
  }
  ngOnInit(): void {
    this.translationService.dictionaryIsLoaded.subscribe(isLoaded => {
      if (isLoaded) {
        this.cd.detectChanges();
      }
    });
  }

  create_payer() {
    const payer = new Payer(new PayerID("ididid"), "Test", "Payer", "1298320ß193ß", "email");
    this.payers.push(payer);
    this.cd.markForCheck();
  }
}
