import { ChangeDetectionStrategy, ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { PayerSerivice } from '../../services/PayerService';
import { Payer, PayerID } from '../../models/payer';
import { PayerDisplayComponent } from './payer-display/payer-display.component';
import { TranslatePipe } from '../../services/TranslationPipe';
import { TranslationService } from '../../services/TranslationService';
import { CommonModule } from '@angular/common';
import { FormBuilder, Validators } from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { ReactiveFormsModule } from '@angular/forms';
import { PayerCreationRequest } from '../../models/requests/PayerCreationRequest';

@Component({
  selector: 'main-menu',
  standalone: true,
  imports: [
    TranslatePipe,
    PayerDisplayComponent,
    CommonModule,
    MatFormFieldModule,
    MatInputModule,
    ReactiveFormsModule 
  ],
  templateUrl: './main-menu.component.html',
  styleUrl: './main-menu.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class MainMenuComponent implements OnInit{
  payers: Payer[] = []
  wantsToCreateNewPayer = false;

  payerForm = this.formBuilder.group({
    firstname: ["", Validators.required],
    lastname: ["", Validators.required],
    telephone: "",
    email: ""
  })

  constructor(
    private payerService: PayerSerivice,
    private translationService: TranslationService,
    private cd: ChangeDetectorRef,
    private formBuilder: FormBuilder  
  ) {
    this.payerService.getAllPayers().then(payers => {
      this.payers = payers;
      this.cd.markForCheck();
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
    if (this.payerForm.valid) {
      const payerRequest = new PayerCreationRequest(
        this.payerForm.value.firstname!,
        this.payerForm.value.lastname!,
        this.payerForm.value.telephone || "",
        this.payerForm.value.email || ""
      );
      this.payerService.saveNewPayer(payerRequest).then(payer => {
        this.payers.push(payer);
      })
    } else {
      console.error("Missing input value");
    }
  }
}
