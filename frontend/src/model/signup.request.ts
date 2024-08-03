export class SignUpRequest {
  constructor(
    public userName: string,
    public coupleName: string,
    public photo: string,
    public email: string,
    public password: string,
    public anniversary: string,
  ) {}
}
