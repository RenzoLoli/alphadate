export class SignUpRequest {
  constructor(
    public username: string,
    public couplename: string,
    public photo: string,
    public email: string,
    public password: string,
    public anniversary: string,
  ) {}
}
