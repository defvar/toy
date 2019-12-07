import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { FormsModule } from '@angular/forms';
import {
  MatInputModule, MatFormFieldModule, MatSidenavModule, MatButtonModule,
  MatToolbarModule, MatIconModule, MatListModule, MatCardModule, MatGridListModule
} from '@angular/material';
import { MatMenuModule } from '@angular/material/menu';
import { LayoutModule } from '@angular/cdk/layout';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { DashboardComponent } from './dashboard/dashboard.component';

@NgModule({
  declarations: [
    AppComponent,
    DashboardComponent
  ],
  imports: [
    BrowserModule,
    FormsModule,
    MatFormFieldModule,
    MatInputModule,
    MatSidenavModule,
    MatToolbarModule,
    MatListModule,
    MatButtonModule,
    MatIconModule,
    MatCardModule,
    MatGridListModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    MatMenuModule,
    LayoutModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
