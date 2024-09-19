import { Injectable } from "@angular/core";

@Injectable({
  providedIn: 'root'
})
export class BrandService {
  getBrandImage() {
    return 'https://wallpapercave.com/wp/wp3249364.jpg';
  }
}
