import { NgModule } from '@angular/core'
import { BrowserModule } from '@angular/platform-browser'

import { AppComponent } from './app.component'
import { IndicatorComponent } from './Indicators/indicator/indicator.component'
import { HomeComponent } from './home/home.component'
import { FormsModule, ReactiveFormsModule } from '@angular/forms'
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { LeftComponent } from './ActionTray/left/left.component';
import { RightComponent } from './ActionTray/right/right.component';
import { ActionsComponent } from './ActionTray/actions/actions.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations'
import { MatInputModule } from "@angular/material/input";
import { MatSelectModule } from "@angular/material/select";

@NgModule({
    declarations: [AppComponent, IndicatorComponent, HomeComponent, LeftComponent, RightComponent, ActionsComponent],
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
    bootstrap: [AppComponent],
})
export class AppModule {}
