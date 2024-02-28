import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PayerDisplayComponent } from './payer-display.component';

describe('PayerDisplayComponent', () => {
  let component: PayerDisplayComponent;
  let fixture: ComponentFixture<PayerDisplayComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PayerDisplayComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(PayerDisplayComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
