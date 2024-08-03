export class User {
  constructor(
    public id: string,
    public userName: string,
    public coupleName: string,
    public photo: string,
    public email: string,
    public anniversary: Date,
  ) {}
}
