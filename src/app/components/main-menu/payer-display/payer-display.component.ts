import { Component, Input, OnInit } from '@angular/core';
import { Payer } from '../../../models/payer';
import { TranslatePipe } from '../../../services/TranslationPipe';
import {MatCardModule} from '@angular/material/card';

@Component({
  selector: 'payer-display',
  standalone: true,
  imports: [
    TranslatePipe,
    MatCardModule
  ],
  templateUrl: './payer-display.component.html',
  styleUrl: './payer-display.component.css'
})
export class PayerDisplayComponent {
  @Input()
  payer!: Payer;
}
