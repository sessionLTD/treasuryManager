import { Component, Input, OnInit } from '@angular/core';
import { Payer } from '../../../models/payer';
import { TranslatePipe } from '../../../services/TranslationPipe';

@Component({
  selector: 'payer-display',
  standalone: true,
  imports: [TranslatePipe],
  templateUrl: './payer-display.component.html',
  styleUrl: './payer-display.component.css'
})
export class PayerDisplayComponent implements OnInit {
  @Input()
  payer!: Payer;

  ngOnInit(): void {
    console.log(this.payer);
  }
}
