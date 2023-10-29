import { ComponentFixture, TestBed } from '@angular/core/testing'

import { IndicatorComponent } from './indicator.component'

describe('IndicatorComponent', () => {
    let component: IndicatorComponent
    let fixture: ComponentFixture<IndicatorComponent>

    beforeEach(() => {
        TestBed.configureTestingModule({
            declarations: [IndicatorComponent],
        })
        fixture = TestBed.createComponent(IndicatorComponent)
        component = fixture.componentInstance
        fixture.detectChanges()
    })

    it('should create', () => {
        expect(component).toBeTruthy()
    })
})
