import { Component } from '@angular/core';
import { FormBuilder, FormGroup } from "@angular/forms";
import {MatSelectModule} from '@angular/material/select';
import {MatFormFieldModule} from '@angular/material/form-field';

@Component({
  selector: 'app-left',
  templateUrl: './left.component.html',
  styleUrls: ['./left.component.css']
})
export class LeftComponent {

   powerAction : FormGroup

  constructor(private fb : FormBuilder) {
     this.powerAction = this.fb.group({
       action: [""]
     })
  }
}
