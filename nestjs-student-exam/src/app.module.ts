import { Module } from '@nestjs/common';
import { ExamModule } from './exam/exam.module';

@Module({
  imports: [ExamModule],
  controllers: [],
  providers: [],
})
export class AppModule { }
