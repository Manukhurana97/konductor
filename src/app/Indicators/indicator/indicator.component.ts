import { Component } from '@angular/core'
import { IndicatorService } from './indicator.service'
import { FormBuilder } from '@angular/forms'
import { faHdd, faMemory, faMicrochip } from '@fortawesome/free-solid-svg-icons'

@Component({
    selector: 'app-indicator',
    templateUrl: './indicator.component.html',
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

  getIconClass(icon: number): string {
    if (icon >= 90) {
      return 'memoryIconHighUsage';
    } else if (icon >= 80) {
      return 'memoryIconHighUsage';
    } else if (icon < 20) {
      return 'memoryIconLowUsage';
    } else {
      return 'memoryIcon';
    }
  }
}
