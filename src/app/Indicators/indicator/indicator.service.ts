import { Injectable } from '@angular/core'
import { invoke } from '@tauri-apps/api/core'

@Injectable({
    providedIn: 'root',
})
export class IndicatorService {
    constructor() {}

    async getCpuUsage(): Promise<number> {
        return await this.getCpuUsageAsync()
    }

    async getRamUsage(): Promise<number> {
        return await this.getRamUsageAsync()
    }

    async getMemoryUsage() {
        return await this.getMemoryUsageAsync()
    }

    async getSystemInfo(): Promise<any> {
        try {
            return await invoke('get_system_info')
        } catch (e) {
            console.error('Error occurred while getting system info', e)
            throw e
        }
    }

    private async getCpuUsageAsync(): Promise<number> {
        try {
            return (await invoke('get_cpu_usage')) as unknown as number
        } catch (e) {
            console.error('Error occured while getting data')
            throw e
        }
    }

    private async getRamUsageAsync(): Promise<number> {
        try {
            return (await invoke('get_ram_usage')) as unknown as number
        } catch (e) {
            console.error('Error occured while getting data')
            throw e
        }
    }

    private async getMemoryUsageAsync(): Promise<number> {
        try {
            return (await invoke('get_memory_usage')) as unknown as number
        } catch (e) {
            console.error('Error occured while getting data')
            throw e
        }
    }
}
