export class Payer {
    constructor(
        public id: PayerID,
        public firstname: String,
        public lastname: String,
        public telephone: String,
        public email: String,
        public needs_to_pay: boolean,
    ) {}
}

export class PayerID {
    constructor(public value: String){};
}