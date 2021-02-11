
insert into `users` (`name`) 
values 
('jean'), 
('pierre');

insert into `sites` (`name`, `location`, `details`)
values 
('Fontainebleau', POINT(48.2435, 2.4209), 'Site de Fontainebleau'), 
('El Cap', POINT(47.223804625757275, -1.6411705517186224), 'Salle d’escalade');

insert into `areas` (`name`, `site_id`)
values
('Rocher Canon', (select(id) from `sites` where `name` = 'Fontainebleau')),
('Bas Cuvier', (select(id) from `sites` where `name` = 'Fontainebleau')),
('Block', (select(id) from `sites` where `name` = 'El Cap')),
('Mur 1', (select(id) from `sites` where `name` = 'El Cap'));

insert into `submissions` (`user_id`, `area_id`, `submission_date`, `status`)
values
((select max(id) from `users` where `name` = 'jean'), (select max(A.id) from `areas` A inner join `sites` S where S.`name` = 'Fontainebleau' and  A.`name` = 'Rocher Canon'), sysdate()-3, 'TREATED');

insert into `photos` (`file_name`, `submission_id`, `path`)
values
('fileA.jpg', (select max(`id`) from `submissions`), '/hostedFiles/path/fileA.jpg'),
('fileB.jpg', (select max(`id`) from `submissions`), '/hostedFiles/path/fileB.jpg'),
('fileC.jpg', (select max(`id`) from `submissions`), '/hostedFiles/path/fileC.jpg'),
('fileD.jpg', (select max(`id`) from `submissions`), '/hostedFiles/path/fileD.jpg');

insert into `submissions` (`user_id`, `area_id`, `submission_date`, `status`)
values
((select max(id) from `users` where `name` = 'pierre'), (select max(A.id) from `areas` A inner join `sites` S where S.`name` = 'El Cap' and  A.`name` = 'Block'), sysdate()-2, 'NEW');

insert into `photos` (`file_name`, `submission_id`, `path`)
values
('fileE.jpg', (select max(`id`) from `submissions`), '/hostedFiles/path/fileE.jpg'),
('fileF.jpg', (select max(`id`) from `submissions`), '/hostedFiles/path/fileF.jpg'),
('fileG.jpg', (select max(`id`) from `submissions`), '/hostedFiles/path/fileG.jpg');

insert into `submissions` (`user_id`, `area_id`, `submission_date`, `status`)
values
((select max(id) from `users` where `name` = 'jean'), (select max(A.id) from `areas` A inner join `sites` S where S.`name` = 'Fontainebleau' and  A.`name` = 'Bas Cuvier'), sysdate()-1, 'NEW');

insert into `photos` (`file_name`, `submission_id`, `path`)
values
('fileH.jpg', (select max(`id`) from `submissions`), '/hostedFiles/path/fileH.jpg'),
('fileI.jpg', (select max(`id`) from `submissions`), '/hostedFiles/path/fileI.jpg'),
('fileJ.jpg', (select max(`id`) from `submissions`), '/hostedFiles/path/fileJ.jpg');

commit;