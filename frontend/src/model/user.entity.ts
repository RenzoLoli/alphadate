export class User {
  constructor(
    public id: string,
    public username: string,
    public couplename: string,
    public photo: string,
    public email: string,
    public anniversary: Date,
  ) {}
}
