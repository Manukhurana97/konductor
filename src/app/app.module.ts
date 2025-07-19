import { NgModule } from '@angular/core'
import { BrowserModule } from '@angular/platform-browser'

import { AppComponent } from './app.component'
import { IndicatorComponent } from './Indicators/indicator/indicator.component'
import { HomeComponent } from './home/home.component'
import { FormsModule, ReactiveFormsModule } from '@angular/forms'
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome'
import { BrowserAnimationsModule } from '@angular/platform-browser/animations'
import { MatInputModule } from '@angular/material/input'
import { MatSelectModule } from '@angular/material/select'
import { NoRightClickDirective } from './no-right-click.directive'

@NgModule({
    declarations: [AppComponent, IndicatorComponent, HomeComponent, NoRightClickDirective],
    imports: [
        BrowserModule,
        ReactiveFormsModule,
        FormsModule,
        FontAwesomeModule,
        BrowserAnimationsModule,
        MatInputModule,
        MatSelectModule,
    ],
    providers: [],
    bootstrap: [],
})
export class AppModule {}
