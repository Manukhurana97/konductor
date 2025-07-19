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
        const sysInfo = await this.indicators.getSystemInfo()
        console.log(sysInfo)
        // Simulate CPU usage as a percentage (replace with real value if available)
        // If you have a get_cpu_usage command, use it here
        const cpuPercent = sysInfo.total_cpu ? Math.round(sysInfo.total_cpu) % 100 : 0
        const ramPercent = sysInfo.total_ram ? Math.round((sysInfo.used_ram / sysInfo.total_ram) * 100) : 0
        const diskPercent = sysInfo.total_disk ? Math.round((sysInfo.used_disk / sysInfo.total_disk) * 100) : 0
        this.indicatorForm.patchValue({
            cpuUsage: cpuPercent,
            ramUsage: ramPercent,
            memoryUsage: diskPercent,
        })
    }

    getIndicatorForm() {
        return this.indicatorForm.value
    }
    getCPU(): number {
        // Always return a value between 0-100 for the ring
        const val = this.indicatorForm.value.cpuUsage
        if (val == null || isNaN(val)) return 0
        return Math.max(0, Math.min(100, val))
    }
    getRam(): number {
        const val = this.indicatorForm.value.ramUsage
        if (val == null || isNaN(val)) return 0
        return Math.max(0, Math.min(100, val))
    }
    getMemory(): number {
        const val = this.indicatorForm.value.memoryUsage
        if (val == null || isNaN(val)) return 0
        return Math.max(0, Math.min(100, val))
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
