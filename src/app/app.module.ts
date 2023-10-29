import { NgModule } from '@angular/core'
import { BrowserModule } from '@angular/platform-browser'

import { AppComponent } from './app.component'
import { IndicatorComponent } from './Indicators/indicator/indicator.component'
import { HomeComponent } from './home/home.component'
import { FormsModule, ReactiveFormsModule } from '@angular/forms'
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome'

@NgModule({
    declarations: [AppComponent, IndicatorComponent, HomeComponent],
    imports: [BrowserModule, ReactiveFormsModule, FormsModule, FontAwesomeModule],
    providers: [],
    bootstrap: [AppComponent],
})
export class AppModule {}
