import { Module } from '@nestjs/common';
import { ClientsModule } from '@nestjs/microservices';
import { ExamController } from './exam.controller';
import { ExamService } from './exam.service';
import { examProducer, ExamQueueService } from './examQueue.service';

@Module({
  imports: [ClientsModule.register([examProducer])],
  controllers: [ExamController],
  providers: [ExamService, ExamQueueService],
})
export class ExamModule { }
