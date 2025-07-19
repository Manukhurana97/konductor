import { Component } from '@angular/core'
import { IndicatorComponent } from '../Indicators/indicator/indicator.component'

@Component({
    selector: 'app-home',
    templateUrl: './home.component.html',
    imports: [IndicatorComponent],
    styleUrls: ['./home.component.css'],
})
export class HomeComponent {}
