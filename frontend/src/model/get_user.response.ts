export class GetUserResponse {
  constructor(
    public id: string,
    public username: string,
    public couplename: string,
    public photo: string,
    public email: string,
    public password: string,
    public anniversary: string,
  ) {}
}
