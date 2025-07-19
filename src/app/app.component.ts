import { Component } from '@angular/core'
import { HomeComponent } from './home/home.component'
import { NoRightClickDirective } from './no-right-click.directive'

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss'],
    imports: [HomeComponent, NoRightClickDirective],
})
export class AppComponent {
    title = 'Konductor'
}
