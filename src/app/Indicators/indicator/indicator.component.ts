import { Component } from '@angular/core'
import { IndicatorService } from './indicator.service'
import { FormBuilder } from '@angular/forms'
import { faHdd, faMemory, faMicrochip } from '@fortawesome/free-solid-svg-icons'
import { FaIconComponent } from '@fortawesome/angular-fontawesome'
import { NgClass } from '@angular/common'

@Component({
    selector: 'app-indicator',
    templateUrl: './indicator.component.html',
    imports: [FaIconComponent, NgClass],
    styleUrls: ['./indicator.component.scss'],
})
export class IndicatorComponent {
    faMicroChip = faMicrochip
    faHdd = faHdd
    faMemory = faMemory

    blocks: number[] = Array.from({ length: 100 }, (_, i) => 3.6 * i)

    indicatorForm = this.fb.group({
        cpuUsage: [0],
        ramUsage: [0],
        memoryUsage: [0],
    })

    constructor(
        private indicators: IndicatorService,
        private fb: FormBuilder
    ) {
        this.getDetails()
    }

    getDetails() {
        setInterval(() => this.updateValues(), 1000)
    }

    async updateValues() {
        this.indicatorForm.patchValue({
            cpuUsage: await this.indicators.getCpuUsage(),
            ramUsage: await this.indicators.getRamUsage(),
            memoryUsage: await this.indicators.getMemoryUsage(),
        })
        console.log(this.indicatorForm.value)
    }

    getIndicatorForm() {
        return this.indicatorForm.value
    }
    getCPU(): number {
        if (this.indicatorForm.value.cpuUsage == null) return 1
        return this.indicatorForm.value.cpuUsage
    }

    getRam(): number {
        if (this.indicatorForm.value.ramUsage == null) return 1
        return this.indicatorForm.value.ramUsage
    }

    getMemory(): number {
        if (this.indicatorForm.value.memoryUsage == null) return 1
        return this.indicatorForm.value.memoryUsage
    }

    getIconClass(type: 'cpu' | 'ram' | 'disk', value: number): string {
        let base = ''
        switch (type) {
            case 'cpu':
                base = 'cpuIcon'
                break
            case 'ram':
                base = 'ramIcon'
                break
            case 'disk':
                base = 'diskIcon'
                break
        }
        if (value >= 90) {
            return base + 'HighUsage'
        } else if (value >= 80) {
            return base + 'HighUsage'
        } else if (value < 20) {
            return base + 'LowUsage'
        } else {
            return base
        }
    }
}
